use std::collections::HashMap;

#[derive(Debug)]
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

	pub fn navigate<'a>(&'a self, path: Vec<&str>) -> Option<&'a DomNode> {
		let mut location = self;

		for node in path {
			match location.children.get(node) {
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

	pub fn add_child(&mut self, child: DomNode) {
		self.children.insert(child.name.clone(), child);
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum RobloxInstance {
	Folder(RobloxFolder),
	ModuleScript(RobloxModuleScript),
	ServerScript(RobloxServerScript),
	LocalScript(RobloxLocalScript),
	Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RobloxFolder;

#[derive(Debug, PartialEq, Eq)]
pub struct RobloxModuleScript {
	pub source: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RobloxServerScript {
	pub source: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RobloxLocalScript {
	pub source: String,
}

#[test]
fn it_finds_nodes() {
	let mut root = DomNode::new("root", RobloxInstance::Unknown);
	let mut container = DomNode::new("container", RobloxInstance::Unknown);
	let child = DomNode::new("child", RobloxInstance::Folder(RobloxFolder {}));

	container.add_child(child);
	root.add_child(container);

	let navigated = root.navigate(vec!["container", "child"]).unwrap();

	assert_eq!(navigated.instance, RobloxInstance::Folder(RobloxFolder {}));
}