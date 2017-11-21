use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct MountPoint {
    pub path: PathBuf,
    pub target: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u64,
    pub verbose: bool,
    pub root_path: PathBuf,
    pub mount_points: HashMap<String, MountPoint>,
}
