mod back;

use std::env::{args};
use crate::back::command::Command;

pub const NULL: &'static str = "null";

fn main() {
    let args : Vec<String> = args().collect();
    match Command::parse_commands(&args) {
        Ok(_) => {}
        Err(e) => {
            println!("- Error: {}", e)
        }
    }
}
