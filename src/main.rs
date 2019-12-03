extern crate dotenv_linter;

use std::process;

fn main() {
    if let Err(e) = dotenv_linter::run() {
        println!("Error: {}", e);

        process::exit(1);
    }
}
