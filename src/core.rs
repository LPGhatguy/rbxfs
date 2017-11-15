use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub port: u64,
    pub verbose: bool,
    pub root_path: PathBuf,
}
