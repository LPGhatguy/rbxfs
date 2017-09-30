use std::path::{Path};
use std::fs::{read_dir, File};
use std::io::Read;

use roblox::{Instance, InstanceDetails};
use roblox::instance_types::{
	RobloxModuleScript,
	RobloxFolder,
};

fn read_file<T: AsRef<Path>>(path: T) -> Option<String> {
	let mut file = match File::open(path.as_ref()) {
		Ok(v) => v,
		Err(_) => {
			return None;
		},
	};

	let mut contents = String::new();

	match file.read_to_string(&mut contents) {
		Ok(_) => {},
		Err(_) => {
			return None;
		},
	}

	Some(contents)
}

/// Attempts to read an instance from the filesystem.
/// Expects an absolute path that has been normalized.
pub fn read_instance_from_path<T: AsRef<Path>>(path: T) -> Option<Instance> {
	let path = path.as_ref();

	let file_name = match path.file_name() {
		Some(name) => {
			match name.to_str() {
				Some(slice) => slice.to_string(),
				None => return None,
			}
		},
		None => return None,
	};

	if path.is_dir() {
		// TODO: check for 'init.lua', etc

		let dir_children = match read_dir(path) {
			Ok(v) => v,
			Err(_) => return None
		};

		let mut instance = Instance::new(file_name, InstanceDetails::Folder(RobloxFolder {}));

		for child in dir_children {
			let child_path = child.unwrap().path();
			let child_instance = match read_instance_from_path(child_path) {
				Some(v) => v,
				None => return None
			};

			instance.add_child(child_instance);
		}

		Some(instance)
	} else if path.is_file() {
		let contents = match read_file(path) {
			Some(v) => v,
			None => return None,
		};

		if file_name.ends_with(".lua") {
			let instance = RobloxModuleScript {
				source: contents,
			};

			// There has to be a better way to do this!
			let instance_name = &file_name[..file_name.len() - 4];

			Some(Instance::new(instance_name, InstanceDetails::ModuleScript(instance)))
		} else {
			None
		}
	} else {
		None
	}
}