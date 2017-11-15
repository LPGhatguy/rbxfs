use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::fmt;

use serde_json;

pub static PROJECT_FILENAME: &'static str = "rbxfs.json";

#[derive(Debug)]
pub enum ProjectLoadError {
    DidNotExist,
    FailedToOpen,
    FailedToRead,
    Invalid,
}

#[derive(Debug)]
pub enum ProjectSaveError {
    FailedToCreate,
}

#[derive(Debug)]
pub enum ProjectInitError {
    AlreadyExists,
    FailedToCreate,
}

impl fmt::Display for ProjectInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ProjectInitError::AlreadyExists => {
                write!(f, "A project already exists at that location.")
            }
            &ProjectInitError::FailedToCreate => {
                write!(f, "Failed to write to the given location.")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Project {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")] serve_port: Option<u64>,
}

impl Project {
    pub fn new() -> Project {
        Project::default()
    }

    pub fn init<T: AsRef<Path>>(location: T) -> Result<(), ProjectInitError> {
        let location = location.as_ref();
        let package_path = location.join(PROJECT_FILENAME);

        match fs::metadata(&package_path) {
            Ok(_) => return Err(ProjectInitError::AlreadyExists),
            Err(_) => {}
        }

        let mut file = match File::create(&package_path) {
            Ok(f) => f,
            Err(_) => return Err(ProjectInitError::FailedToCreate),
        };

        let project = Project::new();
        let serialized = serde_json::to_string_pretty(&project).unwrap();

        file.write(serialized.as_bytes()).unwrap();

        Ok(())
    }

    pub fn save<T: AsRef<Path>>(location: T) -> Result<(), ProjectSaveError> {
        let package_path = location.as_ref().join(Path::new(PROJECT_FILENAME));

        let mut file = match File::create(&package_path) {
            Ok(f) => f,
            Err(_) => return Err(ProjectSaveError::FailedToCreate),
        };

        let project = Project::new();
        let serialized = serde_json::to_string_pretty(&project).unwrap();

        file.write(serialized.as_bytes()).unwrap();

        Ok(())
    }

    pub fn load<T: AsRef<Path>>(location: T) -> Result<Project, ProjectLoadError> {
        let package_path = location.as_ref().join(Path::new(PROJECT_FILENAME));

        match fs::metadata(&package_path) {
            Ok(_) => {}
            Err(_) => return Err(ProjectLoadError::DidNotExist),
        }

        let mut file = match File::open(&package_path) {
            Ok(f) => f,
            Err(_) => return Err(ProjectLoadError::FailedToOpen),
        };

        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(_) => return Err(ProjectLoadError::FailedToRead),
        }

        match serde_json::from_str(&contents) {
            Ok(v) => Ok(v),
            Err(_) => return Err(ProjectLoadError::Invalid),
        }
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
