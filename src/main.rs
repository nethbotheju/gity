use clap::{Command, Arg};
use git2::Repository;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Rust Git Client")
        .version("0.1.0")
        .author("Neth Botheju")
        .about("Custom Git CLI in Rust")
        .subcommand(
            Command::new("init")
                .about("Initialize a new repository")
                .arg(
                    Arg::new("path")
                        .help("Directory to initialize (defaults to current directory)")
                        .default_value(".")
                        .required(false)
                )
        )
        .get_matches();

    if let Some(init_matches) = matches.subcommand_matches("init") {
        let path = init_matches.get_one::<String>("path").unwrap();
        let path = Path::new(path);
        
        match Repository::init(path) {
            Ok(_) => println!("Initialized empty Git repository in {}", path.display()),
            Err(e) => eprintln!("Failed to initialize repository: {}", e),
        }
    }

    Ok(())
}