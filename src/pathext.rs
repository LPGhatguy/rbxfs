use std::env::current_dir;
use std::path::{Path, PathBuf, Component};

/**
 * Turns the path into an absolute one, using the current working directory
 * if necessary.
 */
pub fn canonicalish<T: AsRef<Path>>(value: T) -> PathBuf {
    let value = value.as_ref();

    if value.is_absolute() {
        PathBuf::from(value)
    } else {
        let cwd = current_dir().unwrap();
        cwd.join(value)
    }
}

/**
 * Collapses any `.` values along with any `..` values not at the start of the
 * path.
 */
pub fn collapse<T: AsRef<Path>>(value: T) -> PathBuf {
    let value = value.as_ref();

    let mut buffer = Vec::new();

    for component in value.components() {
        match component {
            Component::ParentDir => {
                match buffer.pop() {
                    Some(_) => {},
                    None => buffer.push(component.as_os_str()),
                }
            },
            Component::CurDir => {},
            _ => {
                buffer.push(component.as_os_str());
            },
        }
    }

    buffer
        .iter()
        .fold(PathBuf::new(), |mut acc, &x| {
            acc.push(x);
            acc
        })
}

#[test]
fn test_collapse() {
    fn identity(buf: PathBuf) {
        assert_eq!(buf, collapse(&buf));
    }

    identity(PathBuf::from("C:\\foo\\bar"));
    identity(PathBuf::from("/a/b/c"));
    identity(PathBuf::from("a/b"));

    assert_eq!(collapse(PathBuf::from("a/b/..")), PathBuf::from("a"));
    assert_eq!(collapse(PathBuf::from("./a/b/c/..")), PathBuf::from("a/b"));
    assert_eq!(collapse(PathBuf::from("../a")), PathBuf::from("../a"));
}
