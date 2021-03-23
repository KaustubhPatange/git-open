mod back;

use std::path::Path;
use std::env::{args};
use crate::back::command::Command;

pub const NULL: &'static str = "null";

fn main() {
    let root = Path::new(".git");
    if root.exists() && root.is_dir() {
        let args : Vec<String> = args().collect();
        match Command::parse_commands(&args) {
            Ok(_) => {}
            Err(e) => {
                println!("- Error: {}", e)
            }
        }
    } else {
        println!("The directory is not git repository")
    }
}
