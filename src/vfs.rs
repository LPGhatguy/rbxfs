use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Represents a virtual layer over multiple parts of the filesystem.
///
/// Paths in this system are represented as slices of strings, and are always
/// relative to a partition, which is an absolute path into the real filesystem.
pub struct Vfs {
    /// Contains all of the partitions mounted by the Vfs.
    ///
    /// These must be absolute paths!
    pub partitions: HashMap<String, PathBuf>,

    /// When the Vfs was initialized; used for change tracking.
    pub start_time: Instant,

    /// A chronologically-sorted list of routes that changed since the Vfs was
    /// created, along with a timestamp denoting when.
    pub change_history: Vec<VfsChange>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VfsChange {
    time: f64,
    route: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum VfsItem {
    File { contents: String },
    Dir { children: HashMap<String, VfsItem> },
}

impl Vfs {
    pub fn new() -> Vfs {
        Vfs {
            partitions: HashMap::new(),
            start_time: Instant::now(),
            change_history: Vec::new(),
        }
    }

    fn route_to_path<R: Borrow<str>>(&self, route: &[R]) -> Option<PathBuf> {
        let (partition_name, rest) = match route.split_first() {
            Some((first, rest)) => (first.borrow(), rest),
            None => return None,
        };

        let partition = match self.partitions.get(partition_name) {
            Some(v) => v,
            None => return None,
        };

        let full_path = {
            let joined = rest.join("/");
            let relative = Path::new(&joined);

            partition.join(relative)
        };

        Some(full_path)
    }

    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<VfsItem, ()> {
        let reader = match fs::read_dir(path) {
            Ok(v) => v,
            Err(_) => return Err(()),
        };

        let mut children = HashMap::new();

        for entry in reader {
            let entry = match entry {
                Ok(v) => v,
                Err(_) => return Err(()),
            };

            let path = entry.path();

            match self.read_path(&path) {
                Ok(child_item) => {
                    let name = path.file_name().unwrap().to_string_lossy().into_owned();

                    children.insert(name, child_item);
                },
                Err(_) => {},
            }
        }

        Ok(VfsItem::Dir {
            children,
        })
    }

    fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<VfsItem, ()> {
        let mut file = match File::open(path) {
            Ok(v) => v,
            Err(_) => return Err(()),
        };

        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Ok(_) => {},
            Err(_) => return Err(()),
        }

        Ok(VfsItem::File {
            contents,
        })
    }

    fn read_path<P: AsRef<Path>>(&self, path: P) -> Result<VfsItem, ()> {
        let path = path.as_ref();

        let metadata = match fs::metadata(path) {
            Ok(v) => v,
            Err(_) => return Err(()),
        };

        if metadata.is_dir() {
            self.read_dir(path)
        } else if metadata.is_file() {
            self.read_file(path)
        } else {
            Err(())
        }
    }

    pub fn current_time(&self) -> f64 {
        let elapsed = self.start_time.elapsed();

        elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9
    }

    pub fn add_change(&mut self, route: Vec<String>) {
        let time = self.current_time();

        self.change_history.push(VfsChange {
            time,
            route,
        });
    }

    pub fn read<R: Borrow<str>>(&self, route: &[R]) -> Result<VfsItem, ()> {
        match self.route_to_path(route) {
            Some(path) => self.read_path(&path),
            None => Err(()),
        }
    }

    pub fn write<R: Borrow<str>>(&self, route: &[R], item: VfsItem) -> Result<(), ()> {
        unimplemented!()
    }

    pub fn delete<R: Borrow<str>>(&self, route: &[R]) -> Result<(), ()> {
        unimplemented!()
    }
}
