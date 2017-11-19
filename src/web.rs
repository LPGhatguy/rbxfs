use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Component, Path};
use std::thread;

use rouille;
use serde;
use serde_json;

use core::Config;

static SERVER_INFO: ServerInfo = ServerInfo {
    server_version: env!("CARGO_PKG_VERSION"),
    protocol_version: 0,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum FsItem {
    File { path: Vec<String>, contents: String },
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

fn json<T: serde::Serialize>(value: T) -> rouille::Response {
    let data = serde_json::to_string(&value).unwrap();
    rouille::Response::from_data("application/json", data)
}

pub fn start(config: Config) {
    let address = format!("localhost:{}", config.port);

    thread::spawn(move || {
        rouille::start_server(address, move |request| {
            router!(request,
				(GET) (/) => {
					json(&SERVER_INFO)
				},

				(GET) (/read_all) => {
                    json(read_all(&config.root_path))
				},

				(POST) (/read) => {
                    rouille::Response::text("Got a read!")
				},

                (POST) (/write) => {
                    rouille::Response::text("Got a write!")
                },

				_ => rouille::Response::empty_404()
			)
        });
    });
}
