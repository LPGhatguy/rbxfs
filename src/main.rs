#[macro_use]
extern crate rouille;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize)]
struct VersionResult {
	version: String,
}

fn main() {
	println!("Listening on localhost:8001");

	rouille::start_server("localhost:8001", |request| {
		router!(request,
			(GET) (/) => {
				rouille::Response::text("rbxfs is up and running!")
			},

			(GET) (/version) => {
				let result = serde_json::to_string(&VersionResult {
					version: "0.3.0".to_string(),
				}).unwrap();

				rouille::Response::text(result)
			},

			(GET) (/read/{object_name: String}) => {
				println!("Read the object {}", object_name);

				rouille::Response::text("got it.")
			},

			_ => rouille::Response::empty_404()
		)
	});
}