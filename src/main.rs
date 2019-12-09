extern crate dotenv_linter;

use std::process;

fn main() {
    if let Err(e) = dotenv_linter::run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
