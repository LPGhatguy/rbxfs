use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u64,
    pub verbose: bool,
    pub root_path: PathBuf,
}
