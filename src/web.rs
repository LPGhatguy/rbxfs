use std::collections::HashMap;
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
    File { path: String, contents: String },
    Dir {
        path: String,
        children: HashMap<String, FsItem>,
    },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerInfo {
    server_version: &'static str,
    protocol_version: u64,
}

fn json<T: serde::Serialize>(value: T) -> rouille::Response {
    let data = serde_json::to_string(&value).unwrap();
    rouille::Response::from_data("application/json", data)
}

pub fn start(config: &Config) {
    let address = format!("localhost:{}", config.port);

    thread::spawn(move || {
        rouille::start_server(address, move |request| {
            router!(request,
				(GET) (/) => {
					json(&SERVER_INFO)
				},

				(GET) (/read_all) => {
					rouille::Response::text("Got a read_all!")
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
