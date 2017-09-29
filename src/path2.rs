use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
pub struct RobloxInstance<'a, 'b> {
	pub name: Cow<'a, str>,
	pub children: HashMap<Cow<'b, str>, RobloxInstance<'a, 'b>>,
	pub details: RobloxInstanceDetails,
}

impl<'a, 'b> RobloxInstance<'a, 'b> {
	pub fn new<T>(name: T, details: RobloxInstanceDetails) -> RobloxInstance<'a, 'b>
		where T: Into<Cow<'a, str>> {

		RobloxInstance {
			name: name.into(),
			children: HashMap::new(),
			details,
		}
	}
}

#[test]
fn it_makes_instances() {
	let name = "hello, world";
	let details = RobloxInstanceDetails::Unknown;
	let child_details = RobloxInstanceDetails::Folder(
		RobloxFolder {
		}
	);

	let mut parent = RobloxInstance::new(name, details);

	let child = RobloxInstance::new(name, child_details);
	parent.children.insert(name.into(), child);
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RobloxInstanceDetails {
	Folder(RobloxFolder),
	ModuleScript(RobloxModuleScript),
	ServerScript(RobloxServerScript),
	LocalScript(RobloxLocalScript),
	Unknown,
}

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