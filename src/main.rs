extern crate clap;
extern crate dotenv_linter;

use std::process;

fn main() {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .version_short("v")
        .get_matches();

    if let Err(e) = dotenv_linter::run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
