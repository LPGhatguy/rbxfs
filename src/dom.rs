use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use std::time::Instant;

use rocket::{Request, Outcome};
use rocket::request::{self, State, FromRequest};
use notify::DebouncedEvent;

use roblox::{Instance, InstanceDetails};
use dom_route::DomRoute;
use fs::read_instance_from_path;

/// Represents the link between the file system and our Instance tree.
#[derive(Debug)]
pub struct Dom {
	root_instance: Instance,
	path: PathBuf,
	start_time: Instant,
	changes: Vec<DomChange>,
}

impl Dom {
	/// Tries to open a Dom pointed at the given object.
	pub fn open<T: AsRef<Path>>(root: T) -> Option<Dom> {
		let root = root.as_ref();

		let root_instance = match read_instance_from_path(root) {
			Some(v) => v,
			None => return None,
		};

		Some(Dom {
			root_instance,
			path: root.to_path_buf(),
			start_time: Instant::now(),
			changes: Vec::new(),
		})
	}

	/// Yields the Dom's current timestamp, used for change tracking.
	pub fn current_time(&self) -> f64 {
		let elapsed = self.start_time.elapsed();

		elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9
	}

	/// Returns all of the changes that have occured since `timestamp`.
	pub fn get_changes_since(&self, timestamp: f64) -> &[DomChange] {
		let mut marker: Option<usize> = None;

		for (index, value) in self.changes.iter().enumerate().rev() {
			if value.timestamp <= timestamp {
				marker = Some(index);
			} else {
				break;
			}
		}

		if let Some(index) = marker {
			&self.changes[index..]
		} else {
			&self.changes[..0]
		}
	}

	pub fn add_change(&mut self, change: DomChange) {
		self.changes.push(change);
	}

	pub fn root(&self) -> &Instance {
		&self.root_instance
	}

	pub fn navigate_mut(&mut self, route: &[String]) -> Option<&mut Instance> {
		self.root_instance.navigate_mut(route)
	}

	pub fn navigate(&self, route: &[String]) -> Option<&Instance> {
		self.root_instance.navigate(route)
	}

	pub fn handle_fs_event(&mut self, event: &DebouncedEvent) {
		println!("fs event: {:?}", event);
		let timestamp = self.current_time();

		match *event {
			DebouncedEvent::Write(ref path) => {
				self.add_change(DomChange {
					route: DomRoute(Vec::new()),
					timestamp,
				});
			},
			DebouncedEvent::Create(ref path) => {
				self.add_change(DomChange {
					route: DomRoute(Vec::new()),
					timestamp,
				});
			},
			DebouncedEvent::Remove(ref path) => {
				self.add_change(DomChange {
					route: DomRoute(Vec::new()),
					timestamp,
				});
			},
			DebouncedEvent::Rename(ref from, ref to) => {
				self.add_change(DomChange {
					route: DomRoute(Vec::new()),
					timestamp,
				});
			},
			_ => {},
		}
	}
}

/// Represents that an instance changed with a timestamp.
#[derive(Debug, Serialize)]
pub struct DomChange {
	route: DomRoute,
	timestamp: f64,
}

/// Represents a globally-accessible mutable Dom.
/// Used to play nicely with Rocket.
#[derive(Debug)]
pub struct DomState(pub Arc<Mutex<Dom>>);

impl<'a, 'r, 'b> FromRequest<'a, 'r> for DomState {
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