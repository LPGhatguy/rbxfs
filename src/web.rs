use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Component, Path};
use std::thread;

use rouille;
use serde;
use serde_json;

use core::Config;
use pathext::absoluteify;
use project::Project;

static MAX_BODY_SIZE: usize = 25 * 1024 * 1025; // 25 MiB

static SERVER_INFO: ServerInfo = ServerInfo {
    server_version: env!("CARGO_PKG_VERSION"),
    protocol_version: 0,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum FsItem {
    File {
        path: Vec<String>,
        contents: String,
    },
    Dir {
        path: Vec<String>,
        children: HashMap<String, FsItem>,
    },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerInfo {
    server_version: &'static str,
    protocol_version: u64,
}

fn get_net_path<A: AsRef<Path>, B: AsRef<Path>>(root: A, source: B) -> Vec<String> {
    let root = root.as_ref();
    let source = source.as_ref();

    assert!(source.is_absolute());
    assert!(source.starts_with(root));

    let relative = source.strip_prefix(root).unwrap();

    let mut result = Vec::new();

    for component in relative.components() {
        match component {
            Component::Normal(name) => {
                let name = name.to_string_lossy().into_owned();

                result.push(name);
            },
            _ => panic!("Not implemented"),
        }
    }

    result
}

fn read<A: AsRef<Path>, B: AsRef<Path>>(root: A, item: B) -> Option<FsItem> {
    let root = root.as_ref();
    let item = item.as_ref();

    let metadata = match fs::metadata(item) {
        Ok(v) => v,
        Err(_) => return None,
    };

    if metadata.is_dir() {
        let reader = match fs::read_dir(item) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let mut children = HashMap::new();

        for entry in reader {
            let entry = match entry {
                Ok(v) => v,
                Err(_) => return None,
            };

            let path = entry.path();

            match read(root, &path) {
                Some(child_item) => {
                    let name = path.file_name().unwrap().to_string_lossy().into_owned();

                    children.insert(name, child_item);
                },
                None => {},
            }
        }

        let net_path = get_net_path(root, item);

        Some(FsItem::Dir {
            path: net_path,
            children,
        })
    } else if metadata.is_file() {
        let mut file = match File::open(item) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Ok(_) => {},
            Err(_) => return None,
        }

        let net_path = get_net_path(root, item);

        Some(FsItem::File {
            path: net_path,
            contents,
        })
    } else {
        None
    }
}

fn read_all<T: AsRef<Path>>(root: T) -> FsItem {
    let root = root.as_ref();

    read(&root, &root).unwrap()
}

fn read_net<T: Borrow<str>>(config: &Config, target: &[T]) -> Option<FsItem> {
    let (mount_name, rest_path) = match target.split_first() {
        Some((first, rest)) => (first.borrow(), rest),
        None => return None,
    };

    let mount = match config.mount_points.get(mount_name) {
        Some(v) => v,
        None => return None,
    };

    let mount_path = absoluteify(&config.root_path, &mount.path);

    let full_path = {
        let joined = rest_path.join("/");
        let relative = Path::new(&joined);

        mount_path.join(relative)
    };

    println!("Mount path {:?}", mount_path);
    println!("Target path {:?}", full_path);

    None
}

fn json<T: serde::Serialize>(value: T) -> rouille::Response {
    let data = serde_json::to_string(&value).unwrap();
    rouille::Response::from_data("application/json", data)
}

fn read_json_text(request: &rouille::Request) -> Option<String> {
    match request.header("Content-Type") {
        Some(header) => if !header.starts_with("application/json") {
            return None;
        },
        None => return None,
    }

    let body = match request.data() {
        Some(v) => v,
        None => return None,
    };

    let mut out = Vec::new();
    match body.take(MAX_BODY_SIZE.saturating_add(1) as u64)
        .read_to_end(&mut out)
    {
        Ok(_) => {},
        Err(_) => return None,
    }

    if out.len() > MAX_BODY_SIZE {
        return None;
    }

    let parsed = match String::from_utf8(out) {
        Ok(v) => v,
        Err(_) => return None,
    };

    Some(parsed)
}

fn read_json<T>(request: &rouille::Request) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    let body = match read_json_text(&request) {
        Some(v) => v,
        None => return None,
    };

    let parsed = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return None,
    };

    Some(parsed)
}

pub fn start(config: Config, project: Project) {
    let address = format!("localhost:{}", config.port);

    thread::spawn(move || {
        rouille::start_server(address, move |request| {
            router!(request,
				(GET) (/) => {
					json(&SERVER_INFO)
				},

                (GET) (/project) => {
                    json(&project)
                },

				(GET) (/read_all) => {
                    // json(read_all(&config.root_path))
                    rouille::Response::text("Nope")
				},

				(POST) (/read) => {
                    let read_request: Vec<Vec<String>> = match read_json(&request) {
                        Some(v) => v,
                        None => return rouille::Response::empty_404(),
                    };

                    json(read_net(&config, &read_request[0]))
				},

                (POST) (/write) => {
                    rouille::Response::text("Got a write!")
                },

				_ => rouille::Response::empty_404()
			)
        });
    });
}
