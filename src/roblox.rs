use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::borrow::{Borrow, Cow};

/// Represents a Roblox instance and all of its properties
/// All instances have a name and children, but most have more.
#[derive(Debug, Serialize, Deserialize)]
pub struct Instance {
	pub name: String,
	pub children: HashMap<String, Instance>,
	pub details: InstanceDetails,
}

impl Instance {
	pub fn new<T>(name: T, details: InstanceDetails) -> Instance
		where T: Into<String> {

		Instance {
			name: name.into(),
			children: HashMap::new(),
			details,
		}
	}

	pub fn add_child(&mut self, child: Instance) {
		self.children.insert(child.name.clone(), child);
	}

	pub fn navigate_mut<T: Borrow<str>>(&mut self, route: &[T]) -> Option<&mut Instance> {
		let mut current_instance = self;

		for route_piece in route {
			let instance = current_instance;
			match instance.children.get_mut(route_piece.borrow()) {
				Some(child_node) => {
					current_instance = child_node;
				},
				None => {
					return None;
				},
			}
		}

		Some(current_instance)
	}

	pub fn navigate<T: Borrow<str>>(&self, route: &[T]) -> Option<&Instance> {
		let mut current_instance = self;

		for route_piece in route {
			match current_instance.children.get(route_piece.borrow()) {
				Some(child_node) => {
					current_instance = child_node;
				},
				None => {
					return None;
				},
			}
		}

		Some(current_instance)
	}
}

#[test]
fn it_makes_instances() {
	let name = "hello, world";
	let details = InstanceDetails::Unknown;
	let child_details = InstanceDetails::Folder(
		instance_types::RobloxFolder {
		}
	);

	let mut parent = Instance::new(name, details);

	let child = Instance::new(name, child_details);
	parent.children.insert(name.into(), child);
}

/// Represents the details of a Roblox object.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InstanceDetails {
	Folder(instance_types::RobloxFolder),
	ModuleScript(instance_types::RobloxModuleScript),
	ServerScript(instance_types::RobloxServerScript),
	LocalScript(instance_types::RobloxLocalScript),
	Unknown,
}

pub mod instance_types {
	#[derive(Debug, Serialize, Deserialize)]
	pub struct RobloxFolder {
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct RobloxModuleScript {
		pub source: String,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct RobloxServerScript {
		pub source: String,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct RobloxLocalScript {
		pub source: String,
	}
}