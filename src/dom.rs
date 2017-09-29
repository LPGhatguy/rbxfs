use std::path::{Component, Path, PathBuf};
use std::fs::{self, File};
use std::io::Read;
use std::time::Instant;
use std::borrow::{Borrow, Cow};

use notify::DebouncedEvent;

use dom_node::{self, DomNode, RobloxInstance};
use path_ext;

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

fn load_node_from_path(path: &Path) -> Result<DomNode, LoadError> {
	let file_name = path.file_name().unwrap().to_string_lossy();

	if path.is_file() {
		if file_name.ends_with(".lua") {
			let source = match read_file(path) {
				Some(v) => v,
				None => {
					return Err(LoadError::FileReadFailure);
				},
			};

			let instance = dom_node::RobloxModuleScript {
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
		let instance = dom_node::RobloxFolder {};
		let mut node = DomNode::new(&file_name, RobloxInstance::Folder(instance));

		let children = match fs::read_dir(path) {
			Ok(v) => v,
			Err(_) => {
				return Err(LoadError::FileReadFailure)
			},
		};

		for child in children {
			let path = child.unwrap().path();
			let child_node = load_node_from_path(path.as_path())?;

			node.children.insert(child_node.name.clone(), child_node);
		}

		Ok(node)
	} else {
		unreachable!()
	}
}

pub fn path_to_dom_path(path: &Path) -> Vec<&str> {
	path
		.components()
		.filter_map(|component| {
			match component {
				Component::Normal(piece) => {
					piece.to_str()
				},
				_ => None,
			}
		})
		.collect::<Vec<_>>()
}

pub fn path_to_string_path(path: &Path) -> Vec<String> {
	path_to_dom_path(path)
		.iter()
		.map(|v| v.to_string())
		.collect::<Vec<_>>()
}

#[derive(Debug)]
pub enum LoadError {
	/// rbxfs couldn't figure out what to do with a file
	UnknownObject,

	/// Couldn't read from one or more files
	FileReadFailure,

	/// Couldn't read from one or more directories
	DirectoryReadFailure,
}

#[derive(Debug, Serialize)]
pub struct DomChange {
	timestamp: f64,
	path: Vec<String>,
}

#[derive(Debug)]
pub struct Dom {
	root_node: DomNode,
	changes: Vec<DomChange>,
	start_time: Instant,
	path: PathBuf,
}

impl Dom {
	pub fn new_from_path(path: &Path) -> Result<Dom, LoadError> {
		let root_node = load_node_from_path(path)?;

		Ok(Dom {
			root_node,
			changes: Vec::new(),
			start_time: Instant::now(),
			path,
		})
	}

	pub fn get_root(&self) -> &DomNode {
		&self.root_node
	}

	pub fn navigate<'a>(&'a self, path: &[&str]) -> Option<&'a DomNode> {
		let mut location = &self.root_node;

		for node in path {
			match location.children.get(*node) {
				Some(v) => {
					location = v;
				},
				None => {
					return None;
				},
			}
		}

		Some(location)
	}

	pub fn write(&mut self, path: &[&str], instance: RobloxInstance) {
		let leading_path = &path[..path.len() - 1];
		let leaf_node_name = &path[path.len() - 1];

		let mut current_node = &mut self.root_node;

		for node_name in leading_path {
			let node = current_node;

			match node.children.get_mut(*node_name) {
				Some(child_node) => {
					current_node = child_node;
				},
				None => {
					println!("Failed to write because we couldn't find a node.");
					return;
				},
			}
		}

		if let Some(child) = current_node.children.get_mut(*leaf_node_name) {
			child.instance = instance;

			return;
		}

		let child = DomNode::new(leaf_node_name, instance);
		current_node.add_child(child);
	}

	pub fn current_time(&self) -> f64 {
		let elapsed = self.start_time.elapsed();

		elapsed.as_secs() as f64 + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0)
	}

	pub fn changes_since(&self, timestamp: f64) -> &[DomChange] {
		let marker: Option<usize> = None;

		for (index, value) in self.changes.iter().enumerate().rev() {
			println!("{}: {:?}", index, value);
		}

		self.changes.as_slice()
	}

	fn canon_path<T: AsRef<Path>>(&self, path: T) -> &Path {
		canonical.as_ref().strip_prefix(&self.path).unwrap()
	}

	pub fn handle_event(&mut self, event: &DebouncedEvent) {
		let now = self.current_time();

		match *event {
			DebouncedEvent::Create(ref path) => {
				let string_path = path_to_string_path(&self.canon_path(&path));

				self.changes.push(DomChange {
					timestamp: now,
					path: string_path.clone(),
				});

				let node = load_node_from_path(&path)
					.unwrap();

				println!("new node: {:?}", node);

				// TODO: create node
			},
			DebouncedEvent::Write(ref path) => {
				let path = self.canon_path(&path);

				self.changes.push(DomChange {
					timestamp: now,
					path: path_to_string_path(&path),
				});

				// todo: create/update node
			},
			DebouncedEvent::Remove(ref path) => {
				let path = self.canon_path(&path);

				self.changes.push(DomChange {
					timestamp: now,
					path: path_to_string_path(&path),
				});

				// todo: remove node
			},
			DebouncedEvent::Rename(ref from_path, ref to_path) => {
				let from_path = self.canon_path(&from_path);
				let to_path = self.canon_path(&to_path);

				self.changes.push(DomChange {
					timestamp: now,
					path: path_to_string_path(&from_path),
				});

				self.changes.push(DomChange {
					timestamp: now,
					path: path_to_string_path(&to_path),
				});

				// todo: move node
			},
			_ => {},
		}

		println!("{:?}", self.changes_since(0.0));
	}
}