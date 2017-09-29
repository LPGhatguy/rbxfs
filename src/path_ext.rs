use std::path::{MAIN_SEPARATOR, Component, Path, PathBuf};
use std::ffi::{OsStr, OsString};
use std::ops::Deref;

use rocket::request::FromSegments;
use rocket::http::uri::Segments;

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

// This probably should be a String + Vec<&str> to avoid a bunch of small allocations
// Lifetimes are much easier with a Vec<String>
#[derive(Debug, Serialize, PartialEq, Eq, Hash, Clone)]
pub struct RbxPath(pub Vec<String>);

impl Deref for RbxPath {
	type Target = Vec<String>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub fn path_to_rbx_path<A: AsRef<Path>, B: AsRef<Path>>(base: A, path: B) -> RbxPath {
	let base = base.as_ref();
	let path = path.as_ref();

	let path = if path.is_relative() {
		path
	} else {
		println!("base: {:?}", base);
		println!("path: {:?}", path);
		path.strip_prefix(base).unwrap()
	};

	let list = path
		.components()
		.filter_map(|component| {
			match component {
				Component::Normal(piece) => {
					piece.to_str()
				},
				_ => None,
			}
		})
		.map(|piece| piece.to_string())
		.collect::<Vec<_>>();

	RbxPath(list)
}

impl<'a> FromSegments<'a> for RbxPath {
	type Error = ();

	fn from_segments(segments: Segments<'a>) -> Result<Self, Self::Error> {
		let mut buffer = Vec::new();

		for segment in segments {
			buffer.push(segment.to_string());
		}

		Ok(RbxPath(buffer))
	}
}

// #[test]
// fn it_converts_absolute_path() {
// 	let base = Path::new("/a/b/c");
// 	let relative = Path::new("/a/b/c/def/ya");

// 	let rbx_path = path_to_rbx_path(base, relative);
// 	let expected = RbxPath(vec!["def".to_string(), "ya".to_string()]);

// 	assert_eq!(rbx_path, expected);
// }