use std::io::Read;
use std::thread;
use std::sync::{Arc, Mutex};

use rouille;
use serde;
use serde_json;

use core::Config;
use project::Project;
use vfs::{Vfs, VfsItem};

static MAX_BODY_SIZE: usize = 25 * 1024 * 1025; // 25 MiB

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerInfo<'a> {
    server_version: &'static str,
    protocol_version: u64,
    server_id: &'a str,
    project: &'a Project,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReadResult<'a> {
    items: Vec<Option<VfsItem>>,
    server_id: &'a str,
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

pub fn start(config: Config, project: Project, vfs: Arc<Mutex<Vfs>>) {
    let address = format!("localhost:{}", config.port);

    let server_id = config.server_id.to_string();

    thread::spawn(move || {
        rouille::start_server(address, move |request| {
            router!(request,
				(GET) (/) => {
					json(ServerInfo {
                        server_version: env!("CARGO_PKG_VERSION"),
                        protocol_version: 0,
                        server_id: &server_id,
                        project: &project,
                    })
				},

				(POST) (/read) => {
                    let read_request: Vec<Vec<String>> = match read_json(&request) {
                        Some(v) => v,
                        None => return rouille::Response::empty_404(),
                    };

                    let items = {
                        let vfs = vfs.lock().unwrap();

                        let mut items = Vec::new();

                        for route in &read_request {
                            match vfs.read(&route) {
                                Ok(v) => items.push(Some(v)),
                                Err(_) => items.push(None),
                            }
                        }

                        items
                    };

                    json(ReadResult {
                        server_id: &server_id,
                        items,
                    })
				},

                (POST) (/write) => {
                    rouille::Response::empty_404()
                },

				_ => rouille::Response::empty_404()
			)
        });
    });
}
