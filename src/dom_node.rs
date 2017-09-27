use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct DomNode {
	pub name: String,
	pub children: HashMap<String, DomNode>,
	pub instance: RobloxInstance,
}

impl DomNode {
	pub fn new(name: &str, instance: RobloxInstance) -> DomNode {
		DomNode {
			name: name.to_string(),
			children: HashMap::new(),
			instance,
		}
	}

	pub fn add_child(&mut self, child: DomNode) {
		self.children.insert(child.name.clone(), child);
	}
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum RobloxInstance {
	Folder(RobloxFolder),
	ModuleScript(RobloxModuleScript),
	ServerScript(RobloxServerScript),
	LocalScript(RobloxLocalScript),
	Unknown,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct RobloxFolder {
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct RobloxModuleScript {
	pub source: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct RobloxServerScript {
	pub source: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct RobloxLocalScript {
	pub source: String,
}