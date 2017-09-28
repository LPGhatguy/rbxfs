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

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RobloxInstance {
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