use clap::{Arg, Command};
use git2::{Cred, Error as GitError, PushOptions, RemoteCallbacks, Repository, StatusOptions, Status};
use rpassword;
use std::error::Error;
use std::io::{self, Write};
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
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("add")
                .about("Add file contents to the index")
                .arg(
                    Arg::new("path")
                        .help("Files to add to the index")
                        .required(true)
                        .num_args(1..),
                ),
        )
        .subcommand(
            Command::new("commit")
                .about("Commit staged changes")
                .arg(
                    Arg::new("message")
                        .short('m')
                        .long("message")
                        .help("Commit message")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("push")
                .about("Push commits to remote repository")
                .arg(
                    Arg::new("remote")
                        .help("Remote repository to push to")
                        .default_value("origin")
                        .required(false),
                )
                .arg(
                    Arg::new("branch")
                        .help("Branch to push")
                        .default_value("main")
                        .required(false),
                ),
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
            if let Some(paths) = add_matches.get_many::<String>("path") {
                if paths.clone().collect::<Vec<_>>() == vec!["."] {
                    let repo = Repository::open(".")?;
                    let mut index = repo.index()?;
            
                    let mut opts = StatusOptions::new();
                    opts.include_untracked(true)
                        .include_ignored(false)
                        .recurse_untracked_dirs(true);
            
                    let statuses = repo.statuses(Some(&mut opts))?;
            
                    for entry in statuses.iter() {
                        if let Some(path) = entry.path() {
                            let status = entry.status();
                            let path = Path::new(path);
            
                            match status {
                                s if s.contains(Status::WT_MODIFIED) || s.contains(Status::WT_NEW) => {
                                    if path.is_dir() {
                                        index.add_all([path], git2::IndexAddOption::CHECK_PATHSPEC, None)
                                            .expect("Error when adding directories");
                                    } else {
                                        index.add_path(path)?;
                                    }
                                    println!("Staged: {}", path.display());
                                }
                                s if s.contains(Status::WT_DELETED) => {
                                    index.remove_path(path)?;
                                    println!("Removed: {}", path.display());
                                }
                                _ => {}
                            }
                        }
                    }
            
                    index.write()?;
                }
            }
            
        }
        Some(("commit", commit_matches)) => {
            let repo = Repository::open(".")?;
            let message = commit_matches.get_one::<String>("message").unwrap();
            
            let mut index = repo.index()?;
            let tree_id = index.write_tree()?;
            let tree = repo.find_tree(tree_id)?;
            
            let signature = repo.signature()?;
            
            let parent_commit = match repo.head() {
                Ok(head) => Some(head.peel_to_commit()?),
                Err(_) => None,
            };
            
            let parents = match &parent_commit {
                Some(commit) => vec![commit],
                None => Vec::new(),
            };

            let commit_id = repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                message,
                &tree,
                &parents,
            )?;
            
            println!("Created commit: {}", commit_id);
        }
        Some(("push", push_matches)) => {
            let repo = Repository::open(".")?;
            let remote_name = push_matches.get_one::<String>("remote").unwrap();
            let branch = push_matches.get_one::<String>("branch").unwrap();
            let mut remote = repo.find_remote(remote_name)?;

            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(|url, username, allowed| {
                if allowed.contains(git2::CredentialType::SSH_KEY) {
                    Cred::ssh_key_from_agent(username.unwrap_or("git"))
                } else if allowed.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
                    
                    print!("Username for {}: ", url);
                    io::stdout().flush().map_err(|e| GitError::from_str(&e.to_string()))?;
                    let mut user_input = String::new();
                    io::stdin()
                        .read_line(&mut user_input)
                        .map_err(|e| GitError::from_str(&e.to_string()))?;
                    let user = user_input.trim();
                    let pass = rpassword::prompt_password("Password: ")
                        .map_err(|e| GitError::from_str(&e.to_string()))?;
                    Cred::userpass_plaintext(user, &pass)
                } else {
                    Err(GitError::from_str("Authentication not supported"))
                }
            });

            let mut push_options = PushOptions::new();
            push_options.remote_callbacks(callbacks);

            let refspec = format!("refs/heads/{}:refs/heads/{}", branch, branch);
            remote.push(&[&refspec], Some(&mut push_options))?;
            println!("Successfully pushed to {}/{}", remote_name, branch);
        }
        _ => {
            println!("Please specify a valid subcommand. Run with --help for usage information.");
        }
    }

    Ok(())
}