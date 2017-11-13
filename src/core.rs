#[derive(Debug)]
pub struct Config {
    pub port: u64,
    pub verbose: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Project {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    serve_port: Option<u64>,
}

impl Project {
    pub fn new() -> Project {
        Project::default()
    }
}

impl Default for Project {
    fn default() -> Project {
        Project {
            name: "some-project".to_string(),
            serve_port: None,
        }
    }
}

pub static PROJECT_FILENAME: &'static str = "rbxfs.json";
