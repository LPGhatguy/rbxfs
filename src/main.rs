#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;
extern crate notify;

mod path_ext;
mod dom;
mod dom_route;
mod roblox;

use dom::{Dom, DomChange, DomState};
use dom_route::{DomRoute};

use std::path::{PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::ops::Deref;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};

use rocket_contrib::Json;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

#[derive(Serialize)]
struct InfoResponse {
	server_version: String,
	protocol_version: String,
}

#[derive(Serialize)]
struct NowResponse {
	current_time: f64,
}

#[derive(Serialize)]
struct ChangedSinceResponse<'a> {
	changes: &'a[DomChange]
}

#[get("/")]
fn root() -> &'static str {
	"rbxfs is running!"
}

#[get("/fs/info")]
fn info() -> Json<InfoResponse> {
	Json(InfoResponse {
		server_version: "1.0.0".to_string(),
		protocol_version: "1.0.0".to_string(),
	})
}

#[get("/fs/now")]
fn now(dom: DomState) -> Json<NowResponse> {
	let dom = dom.lock().unwrap();

	Json(NowResponse {
		current_time: dom.current_time(),
	})
}

#[get("/fs/changes-since/<time>")]
fn changes_since(dom: DomState, time: f64) -> String {
	let dom = dom.lock().unwrap();

	let changes = dom.get_changes_since(time);

	let response = serde_json::to_string(&ChangedSinceResponse {
		changes
	});

	response.unwrap()
}

#[get("/fs/read")]
fn read_root(dom: DomState) -> String {
	println!("Read from root");

	"".to_string()
}

#[get("/fs/read/<path..>")]
fn read(dom: DomState, path: DomRoute) -> String {
	println!("Read from {:?}", path);

	let response = serde_json::to_string(&path);

	response.unwrap()
}

fn main() {
	let mut fs_root = PathBuf::from("test-folder");

	if fs_root.is_relative() {
		fs_root = std::env::current_dir().unwrap().join(fs_root);
	}

	let dom = Arc::new(Mutex::new(Dom::new()));

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

	{
		let dom = dom.clone();

		thread::spawn(move || {
			rocket::custom(config, true)
				.manage(DomState(dom))
				.mount("/", routes![root, info, now, changes_since, read_root, read])
				.launch();
		});
	}

	loop {
		let event = rx.recv().unwrap();

		// let mut dom = dom.lock().unwrap();
		// dom.handle_event(&event);
	}
}