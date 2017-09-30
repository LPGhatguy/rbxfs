use std::ops::Deref;

use rocket::request::FromSegments;
use rocket::http::uri::Segments;

#[derive(Debug, Serialize)]
pub struct DomRoute (pub Vec<String>);

impl<'a> FromSegments<'a> for DomRoute {
	type Error = ();

	fn from_segments(segments: Segments<'a>) -> Result<Self, Self::Error> {
		let mut components = Vec::new();

		for segment in segments {
			components.push(segment.to_string());
		}

		Ok(DomRoute(components))
	}
}

impl Deref for DomRoute {
	type Target = Vec<String>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}