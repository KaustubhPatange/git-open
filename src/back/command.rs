use crate::back::git::Git;
use crate::back::utils::launch_github_uri;
use crate::NULL;
use std::path::Path;

pub  struct Command {}
impl Command {

    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub fn parse_commands(args: &Vec<String>) -> Result<(), String> {
        if args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned()) {
            Command::print_all_commands();
            return Ok(())
        }

        let root = Path::new(".git");
        if root.exists() && root.is_dir() {
            return Err("The directory is not git repository".to_owned());
        }

        return match Git::parse() {
            Ok(git) => {
                if args.len() == 1 {
                    return launch_github_uri(git.url, NULL, NULL)
                }
                let branch_index = args.iter().position(|s| s == "-b").unwrap_or(100);
                let branch_name: String = args.get(branch_index + 1).unwrap_or(&NULL.to_string()).to_string();

                let path_index = args.iter().position(|s| s == "-p").unwrap_or(100);
                let path_name: String = args.get(path_index + 1).unwrap_or(&NULL.to_string()).to_string();

                launch_github_uri(git.url, branch_name.as_str(), path_name.as_str())
            }
            Err(s) => {
                Err(s)
            }
        };
    }

    pub fn print_all_commands() {
        println!("git-open v{} - A command line tool to open git project website of the containing repository in the browser.", Command::VERSION);
        println!("Copyright 2020 Kaustubh Patange - https://github.com/KaustubhPatange/git-open");
        println!();
        println!("Usage: git-open [options]");
        println!();
        println!("Options:");
        println!("      [null]                  Opens the default branch github repository page.");
        println!("      -b, --branch [name]     Opens the github repository page with the specified branch.");
        println!("      -p, --path [file-path]  Opens the github repository page for the file path.");
        println!("      -h, --help              Prints this help message.");
        println!();
        println!("Examples:");
        println!("      git-open");
        println!("      git-open -b dev");
        println!("      git-open -b dev -p path-to-file");
    }
}