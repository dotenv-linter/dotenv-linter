use clap::Arg;
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

    let no_color = args.is_present("no-color");
    if no_color {
        colored::control::set_override(false);
    }
    #[cfg(windows)]
    set_windows_virtual_terminal();

    let warnings = dotenv_linter::run(&args, &current_dir)?;

    let total = warnings.len();

    // Ensure the exit code is 0 if there are no warnings or have been fixed.
    if args.is_present("fix") || total == 0 {
        process::exit(0);
    }

    process::exit(1);
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
        .arg(
            Arg::with_name("fix")
                .short("f")
                .long("fix")
                .help("Automatically fixes warnings"),
        )
        .arg(
            Arg::with_name("no-backup")
                .long("no-backup")
                .help("Prevents .env files from being backed up when modified by -f/--fix"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Doesn't display additional information"),
        )
        .get_matches()
}

#[cfg(windows)]
pub fn set_windows_virtual_terminal() {
    colored::control::set_virtual_terminal(true).ok();
}
