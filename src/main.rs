#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;
extern crate notify;

mod dom_node;
mod dom;

use dom_node::DomNode;
use dom::Dom;

use std::path::{Path, PathBuf};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};

use rocket_contrib::Json;

#[derive(Serialize)]
struct SystemInfo {
	server_version: String,
	protocol_version: String,
}

#[derive(Serialize)]
struct DomResponse {
	ok: bool,
}

#[derive(Debug, Serialize)]
struct DomChange {
	timestamp: f64,
	name: String, // maybe pathbuf?
}

#[derive(Debug, Serialize)]
struct TimeResponse {
	time: f64,
}

#[get("/")]
fn root() -> String {
	"rbxfs is up and running!".to_string()
}

#[get("/fs/info")]
fn info() -> Json<SystemInfo> {
	Json(SystemInfo {
		server_version: "1.0.0".to_string(),
		protocol_version: "1.0.0".to_string(),
	})
}

#[get("/fs/time")]
fn time() -> Json<TimeResponse> {
	Json(TimeResponse {
		time: 0.0
	})
}

#[get("/fs/changed-since/<time>")]
fn changed_since(time: f64) -> Json<DomResponse> {
	Json(DomResponse {
		ok: true
	})
}

#[get("/fs/read/<path..>")]
fn read(path: PathBuf) -> Json<DomResponse> {
	Json(DomResponse {
		ok: true
	})
}

#[post("/fs/write/<path..>")]
fn write(path: PathBuf) -> Json<DomResponse> {
	Json(DomResponse {
		ok: true
	})
}

#[post("/fs/delete/<path..>")]
fn delete(path: PathBuf) -> Json<DomResponse> {
	Json(DomResponse {
		ok: true
	})
}

fn main() {
	let fs_root = Path::new("test-folder");

	let mut dom = Dom::new_from_path(fs_root)
		.expect("Failed to load initial DOM");

	println!("{:?}", dom);

	let config = {
		use rocket::config::{Config, Environment};

		Config::build(Environment::Staging)
			.address("localhost")
			.port(8001)
			.finalize()
			.unwrap()
	};

	let (tx, rx) = mpsc::channel();

	let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))
		.expect("Unable to create watcher!");

	watcher.watch(fs_root, RecursiveMode::Recursive)
		.expect("Unable to watch fs_root!");

	thread::spawn(move || {
		rocket::custom(config, true)
			.mount("/", routes![root, info, time, changed_since, read, write, delete])
			.launch();
	});

	loop {
		let event = rx.recv().unwrap();
		dom.handle_event(&event);
	}
}