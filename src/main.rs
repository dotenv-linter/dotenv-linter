use clap::{AppSettings, Arg, SubCommand};
use std::error::Error;
use std::ffi::OsStr;
use std::{env, process};

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    let current_dir = env::current_dir()?;
    let args = get_args(current_dir.as_os_str());

    if args.is_present("no-color") {
        colored::control::set_override(false);
    }

    match args.subcommand() {
        ("", None) => {
            let total_warnings = dotenv_linter::check(&args, &current_dir)?;

            if total_warnings == 0 {
                process::exit(0);
            }
        }
        ("fix", Some(fix_args)) => {
            dotenv_linter::fix(&fix_args, &current_dir)?;
            process::exit(0);
        }
        ("list", Some(_)) => {
            dotenv_linter::available_check_names()
                .iter()
                .for_each(|name| println!("{}", name));

            process::exit(0);
        }
        _ => {
            eprintln!("unknown command");
        }
    }

    process::exit(1);
}

fn get_args(current_dir: &OsStr) -> clap::ArgMatches {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::VersionlessSubcommands)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .version_short("v")
        .args(common_args(current_dir).as_ref())
        .subcommand(
            SubCommand::with_name("list")
                .setting(AppSettings::ColoredHelp)
                .visible_alias("l")
                .usage("dotenv-linter list")
                .about("Shows list of available checks"),
        )
        .subcommand(
            SubCommand::with_name("fix")
                .setting(AppSettings::ColoredHelp)
                .visible_alias("f")
                .args(common_args(current_dir).as_ref())
                .arg(
                    Arg::with_name("no-backup")
                        .long("no-backup")
                        .help("Prevents backing up .env files"),
                )
                .usage("dotenv-linter fix [FLAGS] [OPTIONS] <input>...")
                .about("Automatically fixes warnings"),
        )
        .get_matches()
}

fn common_args(current_dir: &OsStr) -> Vec<Arg> {
    vec![
        Arg::with_name("input")
            .help("files or paths")
            .index(1)
            .default_value_os(current_dir)
            .required(true)
            .multiple(true),
        Arg::with_name("exclude")
            .short("e")
            .long("exclude")
            .value_name("FILE_NAME")
            .help("Excludes files from check")
            .multiple(true)
            .takes_value(true),
        Arg::with_name("skip")
            .short("s")
            .long("skip")
            .value_name("CHECK_NAME")
            .help("Skips checks")
            .multiple(true)
            .takes_value(true),
        Arg::with_name("recursive")
            .short("r")
            .long("recursive")
            .help("Recursively searches and checks .env files"),
        Arg::with_name("no-color")
            .long("no-color")
            .help("Turns off the colored output"),
        Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .help("Doesn't display additional information"),
    ]
}
