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
mod path_ext;

use dom_node::{DomNode};
use dom::{Dom, DomChange, path_to_dom_path};

use std::path::{Component, Path, PathBuf};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::ops::Deref;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};

use rocket_contrib::Json;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

struct DomState(pub Arc<Mutex<Dom>>);

impl<'a, 'r> FromRequest<'a, 'r> for DomState {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
		let state = request.guard::<State<DomState>>()?;

		Outcome::Success(DomState(state.0.clone()))
	}
}

impl Deref for DomState {
	type Target = Arc<Mutex<Dom>>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[derive(Serialize)]
struct InfoResponse {
	server_version: String,
	protocol_version: String,
}

#[derive(Debug, Serialize)]
struct TimeResponse {
	now: f64,
}

#[derive(Debug, Serialize)]
struct ChangedSinceResponse<'a> {
	now: f64,
	changes: &'a [DomChange],
}

#[derive(Debug, Serialize)]
struct ReadAllResponse<'a> {
	now: f64,
	root: &'a DomNode,
}

#[get("/")]
fn root() -> String {
	"rbxfs is up and running!".to_string()
}

#[get("/fs/info")]
fn info() -> Json<InfoResponse> {
	Json(InfoResponse {
		server_version: "1.0.0".to_string(),
		protocol_version: "1.0.0".to_string(),
	})
}

#[get("/fs/now")]
fn now(dom: DomState) -> Json<TimeResponse> {
	let dom = dom.lock().unwrap();
	Json(TimeResponse {
		now: dom.current_time(),
	})
}

#[get("/fs/changed-since/<time>")]
fn changed_since(dom: DomState, time: f64) -> String {
	let dom = dom.lock().unwrap();

	let changes = dom.changes_since(time);
	let now = dom.current_time();

	let response = ChangedSinceResponse {
		changes,
		now,
	};

	let result = serde_json::to_string(&response).unwrap();

	result
}

#[get("/fs/read-all")]
fn read_all(dom: DomState) -> String {
	let dom = dom.lock().unwrap();

	let root = dom.get_root();
	let now = dom.current_time();

	let response = ReadAllResponse {
		root,
		now,
	};

	let result = serde_json::to_string(&response).unwrap();

	result
}

#[get("/fs/read/<path..>")]
fn read(dom: DomState, path: PathBuf) -> String {
	let dom = dom.lock().unwrap();

	let path = path_to_dom_path(path.as_path());
	let node = dom.navigate(&path);

	println!("Got node: {:?}", node);

	let result = serde_json::to_string(&node).unwrap();

	result
}

fn main() {
	let fs_root = Path::new("test-folder");

	let dom = {
		let dom = Dom::new_from_path(fs_root)
			.expect("Failed to load initial DOM");

		println!("{:?}", dom);

		Arc::new(Mutex::new(dom))
	};

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
				.mount("/", routes![root, info, now, changed_since, read_all, read])
				.launch();
		});
	}

	loop {
		let event = rx.recv().unwrap();

		let mut dom = dom.lock().unwrap();
		dom.handle_event(&event);
	}
}