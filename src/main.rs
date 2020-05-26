extern crate clap;
extern crate dotenv_linter;

use std::process;

fn main() {
    match dotenv_linter::run() {
        Ok(warnings) => {
            let mut all_fixed = true;
            warnings.iter().for_each(|w| {
                let is_fixed = w.is_fixed.unwrap_or(false);
                if !is_fixed {
                    all_fixed = false;
                }

                println!("{}", w);
            });
            if all_fixed {
                process::exit(0);
            }
        }
        Err(error) => {
            eprintln!("dotenv-linter: {}", error);
        }
    };

    process::exit(1);
}
