extern crate clap;
extern crate dotenv_linter;

use clap::Arg;
use std::process;

fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .version_short("v")
        .arg(
            Arg::with_name("include")
                .short("i")
                .long("include")
                .value_name("FILE_NAME")
                .help("Includes a file to check")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("exclude")
                .short("e")
                .long("exclude")
                .value_name("FILE_NAME")
                .help("Excludes a file from check")
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();

    if let Err(e) = dotenv_linter::run(matches) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
