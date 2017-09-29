use std::path::{MAIN_SEPARATOR, Component, Path, PathBuf};
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};

fn join_os_str(buffer: &[&OsStr], sep: &OsStr) -> OsString {
	let len = buffer.len();
	let mut result = OsString::new();

	for (i, piece) in buffer.iter().enumerate() {
		result.push(piece);

		if i < len - 1 {
			result.push(sep);
		}
	}

	result
}

#[test]
fn it_joins_os_str() {
	let source = vec![
		OsString::from("Hello"),
		OsString::from("my"),
		OsString::from("world!"),
	];
	let separator = OsString::from(" ");
	let sliced = source
		.iter()
		.map(OsString::as_os_str)
		.collect::<Vec<_>>();

	let result = join_os_str(&sliced, separator.as_os_str());

	assert_eq!(result, OsString::from("Hello my world!"));
}

/// Collapses .. with no regard for symlinks.
pub fn normalize<T: AsRef<Path>>(path: T) -> PathBuf {
	let path = path.as_ref();
	let mut buffer: Vec<&OsStr> = Vec::new();

	for component in path.components() {
		match component {
			Component::CurDir => {},
			Component::ParentDir => {
				// TODO: check length of buffer
				buffer.pop();
			},
			_ => {
				buffer.push(component.as_os_str());
			}
		}
	}

	// This allocates and isn't ideal, hm.
	let separator = MAIN_SEPARATOR.to_string();

	let joined = join_os_str(&buffer, OsStr::new(&separator));

	PathBuf::from(joined)
}

#[test]
fn it_keeps_stuff_the_same() {
	let source = Path::new("C:\\windows\\foo");
	let normalized = normalize(source);

	assert_eq!(normalized.as_path(), source);
}

#[test]
fn it_collapses_dot_dot() {
	let source = Path::new("/foo/bar/..");
	let normalized = normalize(source);

	assert_eq!(normalized.as_path(), Path::new("/foo"));
}