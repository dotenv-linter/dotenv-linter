extern crate clap;
extern crate dotenv_linter;

use std::process;

fn main() {
    match dotenv_linter::new().run() {
        Ok(warnings) => {
            if !warnings.is_empty() {
                warnings.iter().for_each(|w| println!("{}", w));
            }
        }
        Err(error) => {
            eprintln!("dotenv-linter: {}", error);
        }
    };

    process::exit(1);
}
