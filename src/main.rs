use clap::{Command, Arg};
use git2::Repository;
use std::error::Error;
use std::path::Path;
use std::fs;

fn add_directory_recursively(dir_path: &str, index: &mut git2::Index) -> Result<(), Box<dyn Error>> {
    let path = Path::new(dir_path);
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        // Skip .git directory and hidden files/directories
        let path_str = path.to_str().unwrap_or("");
        if path_str.contains("/.git/") || path_str.contains("\\.git\\") || 
           path.file_name().and_then(|s| s.to_str()).map_or(false, |s| s.starts_with(".")) {
            continue;
        }
        
        if path.is_dir() {
            // Recursively add files from subdirectory
            add_directory_recursively(path_str, index)?;
        } else if path.is_file() {
            // Strip the leading ./ if present
            let clean_path = path.strip_prefix("./").unwrap_or(&path);
            match index.add_path(clean_path) {
                Ok(_) => println!("Added file: {}", clean_path.display()),
                Err(e) => eprintln!("Failed to add {}: {}", clean_path.display(), e),
            }
        }
    }
    
    Ok(())
}

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
        .subcommand(
            Command::new("commit")
                .about("Commit staged changes")
                .arg(
                    Arg::new("message")
                        .short('m')
                        .long("message")
                        .help("Commit message")
                        .required(true)
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
                    add_directory_recursively(".", &mut index)?;
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
        Some(("commit", commit_matches)) => {
            let repo = Repository::open(".")?;
            let message = commit_matches.get_one::<String>("message").unwrap();
            
            // Get the index and write it to a tree
            let mut index = repo.index()?;
            let tree_id = index.write_tree()?;
            let tree = repo.find_tree(tree_id)?;
            
            // Get signature for the commit
            let signature = repo.signature()?;
            
            // Get the parent commit (HEAD)
            let parent_commit = match repo.head() {
                Ok(head) => Some(head.peel_to_commit()?),
                Err(_) => None, // Handle initial commit (no parent)
            };
            
            let parents = match &parent_commit {
                Some(commit) => vec![commit],
                None => Vec::new(),
            };

            // Create the commit
            let commit_id = repo.commit(
                Some("HEAD"),      // Reference to update
                &signature,        // Author
                &signature,        // Committer
                message,           // Message
                &tree,             // Tree
                &parents,          // Parents
            )?;
            
            println!("Created commit: {}", commit_id);
        }
        _ => {
            println!("Please specify a valid subcommand. Run with --help for usage information.");
        }
    }

    Ok(())
}