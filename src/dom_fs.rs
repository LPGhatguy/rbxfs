use std::path::Path;
use std::fs::{self, File};
use std::io::Read;

use dom::{self, DomNode, RobloxInstance};

#[derive(Debug)]
pub enum LoadError {
	/// rbxfs couldn't figure out what to do with a file
	UnknownObject,

	/// Couldn't read from one or more files
	FileReadFailure,

	/// Couldn't read from one or more directories
	DirectoryReadFailure,
}

pub trait LoadFromPath where Self: Sized {
	fn load_from_path(path: &Path) -> Result<Self, LoadError>;
}

fn read_file(path: &Path) -> Option<String> {
	let mut f = match File::open(path) {
		Ok(v) => v,
		Err(_) => {
			return None;
		},
	};

	let mut contents = String::new();

	match f.read_to_string(&mut contents) {
		Ok(_) => {},
		Err(_) => {
			return None;
		},
	}

	Some(contents)
}

fn without_suffix<'a, 'b>(source: &'a str, suffix: &'b str) -> &'a str {
	if source.ends_with(suffix) {
		&source[0..(source.len() - suffix.len())]
	} else {
		source
	}
}

impl LoadFromPath for DomNode {
	fn load_from_path(path: &Path) -> Result<DomNode, LoadError> {
		let file_name = path.file_name().unwrap().to_string_lossy();

		if path.is_file() {
			if file_name.ends_with(".lua") {
				let source = match read_file(path) {
					Some(v) => v,
					None => {
						return Err(LoadError::FileReadFailure);
					},
				};

				let instance = dom::RobloxModuleScript {
					source
				};

				let instance_name = without_suffix(&file_name, ".lua");

				Ok(DomNode::new(&instance_name, RobloxInstance::ModuleScript(instance)))
			} else if file_name.ends_with(".model.json") {
				Ok(DomNode::new(&file_name, RobloxInstance::Unknown))
			} else {
				Err(LoadError::UnknownObject)
			}
		} else if path.is_dir() {
			Ok(DomNode::new("", RobloxInstance::Unknown))
		} else {
			unreachable!()
		}
	}
}