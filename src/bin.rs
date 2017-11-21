#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rouille;

#[macro_use]
extern crate clap;

extern crate notify;
extern crate serde;
extern crate serde_json;
extern crate rand;

pub mod web;
pub mod core;
pub mod project;
pub mod pathext;
pub mod vfs;
pub mod vfs_watch;

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

use core::Config;
use pathext::canonicalish;
use project::Project;
use vfs::Vfs;
use vfs_watch::VfsWatcher;

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

    let server_id = rand::random::<u64>();

    if verbose {
        println!("Server ID: {}", server_id);
    }

    match matches.subcommand() {
        ("init", sub_matches) => {
            let sub_matches = sub_matches.unwrap();
            let project_path = Path::new(sub_matches.value_of("PATH").unwrap_or("."));
            let full_path = canonicalish(project_path);

            match Project::init(&full_path) {
                Ok(_) => {
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

            if verbose {
                println!("Attempting to locate project at {}", project_path.display());
            }

            let project = match Project::load(&project_path) {
                Ok(v) => {
                    println!("Using project from {}", project_path.display());
                    v
                },
                Err(_) => {
                    println!("Using default project...");
                    Project::default()
                },
            };

            let port = {
                match sub_matches.value_of("port") {
                    Some(source) => match source.parse::<u64>() {
                        Ok(value) => value,
                        Err(_) => {
                            eprintln!("Invalid port '{}'", source);
                            std::process::exit(1);
                        },
                    },
                    None => project.serve_port,
                }
            };

            let config = Config {
                port,
                verbose,
                server_id,
            };

            if verbose {
                println!("Loading VFS...");
            }

            let vfs = {
                let mut vfs = Vfs::new();

                for (name, project_partition) in &project.partitions {
                    let path = {
                        let given_path = Path::new(&project_partition.path);

                        if given_path.is_absolute() {
                            given_path.to_path_buf()
                        } else {
                            project_path.join(given_path)
                        }
                    };

                    if verbose {
                        println!("Partition '{}': {} @ {}", name, project_partition.target, project_partition.path);
                    }

                    vfs.partitions.insert(name.clone(), path);
                }

                Arc::new(Mutex::new(vfs))
            };

            {
                let vfs = vfs.clone();

                thread::spawn(move || {
                    let mut watcher = VfsWatcher::new(vfs);

                    watcher.start();
                });
            }

            web::start(config.clone(), project.clone(), vfs.clone());

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
