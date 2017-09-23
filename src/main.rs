#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;

mod dom;
mod dom_fs;

use dom::DomNode;
use dom_fs::LoadFromPath;

use std::path::Path;

use rocket_contrib::Json;

#[derive(Serialize)]
struct SystemInfo {
	server_version: String,
	protocol_version: String,
}

#[get("/")]
fn root() -> String {
	"rbxfs is up and running!".to_string()
}

#[get("/rbxfs")]
fn info() -> Json<SystemInfo> {
	Json(SystemInfo {
		server_version: "1.0.0".to_string(),
		protocol_version: "1.0.0".to_string(),
	})
}

fn main() {
	let root = dom::DomNode::load_from_path(Path::new("test-folder"));

	println!("{:?}", root);

	let config = {
		use rocket::config::{Config, Environment};

		Config::build(Environment::Staging)
			.address("localhost")
			.port(8001)
			.finalize()
			.unwrap()
	};

	rocket::custom(config, true)
		.mount("/", routes![root, info])
		.launch();
}