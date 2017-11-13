use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::fmt;

use serde_json;

use core::{Project, PROJECT_FILENAME};

#[derive(Debug)]
pub enum InitError {
    AlreadyExists,
    FailedToCreate,
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &InitError::AlreadyExists => {
                write!(f, "A project already exists at that location.")
            },
            &InitError::FailedToCreate => {
                write!(f, "Failed to write to the given location.")
            },
        }
    }
}

pub fn init<P: AsRef<Path>>(dir: P) -> Result<(), InitError> {
    let dir = dir.as_ref();
    let package_path = dir.join(PROJECT_FILENAME);

    match fs::metadata(&package_path) {
        Ok(_) => return Err(InitError::AlreadyExists),
        Err(_) => {},
    }

    let mut file = match File::create(&package_path) {
        Ok(f) => f,
        Err(_) => return Err(InitError::FailedToCreate),
    };

    let project = Project::new();
    let serialized = serde_json::to_string_pretty(&project).unwrap();

    file.write(serialized.as_bytes()).unwrap();

    Ok(())
}
