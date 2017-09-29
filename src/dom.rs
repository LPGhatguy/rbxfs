use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::time::Instant;

use rocket::{Request, Outcome};
use rocket::request::{self, State, FromRequest};

use roblox::{Instance, InstanceDetails};
use dom_route::DomRoute;

/// Represents the link between the file system and our Instance tree.
#[derive(Debug)]
pub struct Dom {
	root_instance: Instance,
	path: PathBuf,
	start_time: Instant,
	changes: Vec<DomChange>,
}

impl Dom {
	/// Creates a new Dom.
	pub fn new() -> Dom {
		Dom {
			root_instance: Instance::new("game", InstanceDetails::Unknown),
			path: PathBuf::from(""),
			start_time: Instant::now(),
			changes: Vec::new(),
		}
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