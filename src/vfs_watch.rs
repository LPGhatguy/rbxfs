use std::thread;
use std::time;
use std::sync::{Arc, Mutex};

use vfs::Vfs;

pub struct VfsWatcher {
    vfs: Arc<Mutex<Vfs>>,
}

impl VfsWatcher {
    pub fn new(vfs: Arc<Mutex<Vfs>>) -> VfsWatcher {
        VfsWatcher {
            vfs,
        }
    }

    pub fn start(&mut self) {
        loop {
            // Stub to lock the VFS for a brief moment every 200 ms
            {
                let mut vfs = self.vfs.lock().unwrap();
            }

            thread::sleep(time::Duration::from_millis(200));
        }
    }
}
