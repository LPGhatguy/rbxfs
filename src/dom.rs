use std::path::Path;
use std::fs::{self, File};
use std::io::Read;
use std::time::Instant;

use notify::DebouncedEvent;

use dom_node::{self, DomNode, RobloxInstance};

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
		Ok(DomNode::new("", RobloxInstance::Unknown))
	} else {
		unreachable!()
	}
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

#[derive(Debug)]
pub enum DomChangeDetails {
	Created {
		path: Vec<String>,
	},
	Deleted {
		path: Vec<String>,
	},
	Changed {
		path: Vec<String>,
	},
	Renamed {
		from: Vec<String>,
		to: Vec<String>,
	},
	Unknown, // temp
}

#[derive(Debug)]
pub struct DomChange {
	timestamp: f64,
	details: DomChangeDetails,
}

#[derive(Debug)]
pub struct Dom {
	root_node: DomNode,
	changes: Vec<DomChange>,
	start_time: Instant,
}

impl Dom {
	pub fn new_from_path(path: &Path) -> Result<Dom, LoadError> {
		let root_node = load_node_from_path(path)?;

		Ok(Dom {
			root_node,
			changes: Vec::new(),
			start_time: Instant::now(),
		})
	}

	pub fn current_time(&self) -> f64 {
		let elapsed = self.start_time.elapsed();

		elapsed.as_secs() as f64 + (elapsed.subsec_nanos() as f64) / 1_000_000.0
	}

	pub fn changes_since(&self, timestamp: f64) -> &[DomChange] {
		let marker: Option<usize> = None;

		for (index, value) in self.changes.iter().enumerate().rev() {
			println!("{}: {:?}", index, value);
		}

		self.changes.as_slice()
	}

	pub fn handle_event(&mut self, event: &DebouncedEvent) {
		let now = self.current_time();

		match *event {
			DebouncedEvent::Create(ref path) => {
				self.changes.push(DomChange {
					timestamp: now,
					details: DomChangeDetails::Unknown,
				});
			},
			DebouncedEvent::Write(ref path) => {
				self.changes.push(DomChange {
					timestamp: now,
					details: DomChangeDetails::Unknown,
				});
			},
			DebouncedEvent::Remove(ref path) => {
				self.changes.push(DomChange {
					timestamp: now,
					details: DomChangeDetails::Unknown,
				});
			},
			DebouncedEvent::Rename(ref from_path, ref to_path) => {
				self.changes.push(DomChange {
					timestamp: now,
					details: DomChangeDetails::Unknown,
				});
			},
			_ => {},
		}

		println!("{:?}", self.changes_since(0.0));
	}
}