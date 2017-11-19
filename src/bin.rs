#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rouille;

#[macro_use]
extern crate clap;

extern crate notify;
extern crate serde;
extern crate serde_json;

mod web;
mod core;
mod project;
mod pathext;

use std::path::{Component, Path, PathBuf};

use core::Config;
use pathext::canonicalish;
use project::Project;

fn main() {
    let matches = clap_app!(rbxfs =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))

        (@subcommand init =>
            (about: "Creates a new rbxfs project")
            (@arg PATH: "Path to the place to create the project. Defaults to the current directory.")
        )

        (@subcommand serve =>
            (about: "Serves the project's files for use with the rbxfs dev plugin.")
            (@arg PROJECT: "Path to the project to serve. Defaults to the current directory.")
            (@arg port: --port +takes_value "The port to listen on. Defaults to 8000.")
        )

        (@subcommand pack =>
            (about: "Packs the project into a GUI installer bundle.")
            (@arg PROJECT: "Path to the project to pack. Defaults to the current directory.")
        )

        (@arg verbose: --verbose "Enable extended logging.")
    ).get_matches();

    let verbose = match matches.occurrences_of("verbose") {
        0 => false,
        _ => true,
    };

    match matches.subcommand() {
        ("init", sub_matches) => {
            let sub_matches = sub_matches.unwrap();
            let path = Path::new(sub_matches.value_of("PATH").unwrap_or("."));

            match Project::init(&path) {
                Ok(_) => {
                    let full_path = canonicalish(path);
                    println!("Created new empty project at {}", full_path.display());
                },
                Err(e) => {
                    eprintln!("Failed to create new project.\n{}", e);
                    std::process::exit(1);
                },
            }
        },
        ("serve", sub_matches) => {
            let sub_matches = sub_matches.unwrap();

            let project_path = match sub_matches.value_of("PROJECT") {
                Some(v) => PathBuf::from(v),
                None => std::env::current_dir().unwrap(),
            };

            let project = Project::load(&project_path).unwrap_or(Project::default());

            println!("Loaded project: {:?}", project);

            let port = {
                match sub_matches.value_of("port") {
                    Some(source) => match source.parse::<u64>() {
                        Ok(value) => value,
                        Err(_) => {
                            eprintln!("Invalid port '{}'", source);
                            std::process::exit(1);
                        },
                    },
                    None => 8000,
                }
            };

            let config = Config {
                port,
                verbose,
                root_path: std::env::current_dir().unwrap(),
            };

            web::start(config.clone());

            println!("Server listening on port {}", port);

            loop {}
        },
        ("pack", _) => {
            eprintln!("Not implemented.");
            std::process::exit(1);
        },
        _ => {
            eprintln!("Please specify a subcommand!");
            eprintln!("Try 'rbxfs help' for information.");
            std::process::exit(1);
        },
    }
}
