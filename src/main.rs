use clap::Arg;
use colored::*;
use std::error::Error;
use std::ffi::OsStr;
use std::{env, process};

fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let args = get_args(current_dir.as_os_str());

    if args.is_present("show-checks") {
        dotenv_linter::available_check_names()
            .iter()
            .for_each(|name| println!("{}", name));
        process::exit(0);
    }

    let warnings = dotenv_linter::run(&args, &current_dir)?;
    if warnings.is_empty() {
        process::exit(0);
    }

    print_warnings(args.is_present("no-color"), warnings);
    process::exit(1)
}

fn print_warnings(no_color: bool, warnings: Vec<dotenv_linter::common::Warning>) {
    if no_color {
        warnings.iter().for_each(|w| {
            println!(
                "{}:{} {}: {}",
                w.line.file.path.display(),
                w.line.number,
                w.check_name,
                w.message
            )
        });
        println!("{} problems", warnings.len());
    } else {
        warnings.iter().for_each(|w| {
            println!(
                "{}:{} {} {}",
                w.line.file.path.display().to_string().italic(),
                w.line.number.to_string().italic(),
                w.check_name.red().bold(),
                w.message
            );
        });
        println!();
        println!(
            "{} {} {}",
            "\u{2717}".red().bold(),
            warnings.len().to_string().red().bold(),
            "problems".red().bold()
        );
        println!();
    }
}

fn get_args(current_dir: &OsStr) -> clap::ArgMatches {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .version_short("v")
        .arg(
            Arg::with_name("input")
                .help("files or paths")
                .index(1)
                .default_value_os(current_dir)
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("exclude")
                .short("e")
                .long("exclude")
                .value_name("FILE_NAME")
                .help("Excludes files from check")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("skip")
                .short("s")
                .long("skip")
                .value_name("CHECK_NAME")
                .help("Skips checks")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("show-checks")
                .long("show-checks")
                .help("Shows list of available checks"),
        )
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .long("recursive")
                .help("Recursively search and check .env files"),
        )
        .arg(
            Arg::with_name("no-color")
                .long("no-color")
                .help("Turns off the colored output"),
        )
        .get_matches()
}
