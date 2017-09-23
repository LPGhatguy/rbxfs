use std::path::Path;
use std::fs::{self, File};
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub enum VfsItem {
	Folder(VfsFolder),
	Script(VfsScript),
}

#[derive(Debug, Serialize)]
pub struct VfsFolder {
	pub name: String,
	pub children: HashMap<String, VfsItem>,
}

#[derive(Debug, Serialize)]
pub enum VfsScriptKind {
	Script,
	LocalScript,
	ModuleScript,
}

#[derive(Debug, Serialize)]
pub struct VfsScript {
	pub name: String,
	pub kind: VfsScriptKind,
	pub source: String,
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

impl VfsItem {
	pub fn load_from_path(path: &Path) -> Option<VfsItem> {
		if path.is_file() {
			let name = path.file_name().unwrap().to_string_lossy().into();
			let source = read_file(path).unwrap();

			let script = VfsScript {
				name,
				source,
				kind: VfsScriptKind::ModuleScript,
			};

			Some(VfsItem::Script(script))
		} else if path.is_dir() {
			let name = match path.file_name() {
				Some(v) => v.to_string_lossy().into(),
				None => {
					return None;
				},
			};

			let mut folder = VfsFolder {
				name,
				children: HashMap::new(),
			};

			let children = match fs::read_dir(path) {
				Ok(v) => v,
				Err(_) => {
					return None;
				},
			};

			for child in children {
				let path = child.unwrap().path();
				let vfs_child = VfsItem::load_from_path(&path).unwrap();

				folder.children.insert(vfs_child.get_name().clone(), vfs_child);
			}

			Some(VfsItem::Folder(folder))
		} else {
			None
		}
	}

	pub fn navigate(&self, path: Vec<&str>) -> Option<&VfsItem> {
		let mut location = self;

		println!("Navigate from {:?}", self);

		for node in path {
			match *location {
				VfsItem::Folder(ref folder) => {
					match folder.children.get(node) {
						Some(child) => {
							location = child;
						},
						None => {
							println!("Folder didn't contain {:?}", node);
							return None;
						},
					}
				},
				VfsItem::Script(ref script) => {
					println!("Parent {:?} was a script", node);
					return None;
				},
			}
		}

		Some(location)
	}

	pub fn get_name(&self) -> &String {
		match *self {
			VfsItem::Folder(ref folder) => &folder.name,
			VfsItem::Script(ref script) => &script.name,
		}
	}
}