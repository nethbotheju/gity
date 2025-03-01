use clap::{Command, Arg};
use git2::Repository;
use std::error::Error;
use std::path::Path;
use std::fs;

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
        .subcommand(
            Command::new("add")
                .about("Add file contents to the index")
                .arg(
                    Arg::new("path")
                        .help("Files to add to the index")
                        .required(true)
                        .num_args(1..)
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let path = init_matches.get_one::<String>("path").unwrap();
            let path = Path::new(path);
            
            match Repository::init(path) {
                Ok(_) => println!("Initialized empty Git repository in {}", path.display()),
                Err(e) => eprintln!("Failed to initialize repository: {}", e),
            }
        }
        Some(("add", add_matches)) => {
            let repo = Repository::open(".")?;
            let paths = add_matches.get_many::<String>("path")
                .unwrap()
                .collect::<Vec<_>>();

            let mut index = repo.index()?;
            
            for path_str in paths {
                let path = Path::new(path_str);
                
                if path.to_str() == Some(".") {
                    // Handle adding all files in current directory
                    for entry in fs::read_dir(".")? {
                        let entry = entry?;
                        let path = entry.path();
                        // Skip .git directory and hidden files
                        if path.is_file() && !path.to_str().unwrap_or("").contains("/.git/") {
                            // Strip the leading ./ if present
                            let clean_path = path.strip_prefix("./").unwrap_or(&path);
                            match index.add_path(clean_path) {
                                Ok(_) => println!("Added file: {}", clean_path.display()),
                                Err(e) => eprintln!("Failed to add {}: {}", clean_path.display(), e),
                            }
                        }
                    }
                } else {
                    // Handle specific file paths
                    let clean_path = path.strip_prefix("./").unwrap_or(path);
                    match index.add_path(clean_path) {
                        Ok(_) => println!("Added file: {}", clean_path.display()),
                        Err(e) => eprintln!("Failed to add {}: {}", clean_path.display(), e),
                    }
                }
            }
            
            index.write()?;
        }
        _ => {}
    }

    Ok(())
}