use clap::{App, AppSettings, Arg, SubCommand};
use std::ffi::OsStr;

pub fn new(current_dir: &OsStr) -> App {
    App::new(env!("CARGO_PKG_NAME"))
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::VersionlessSubcommands)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .version_short("v")
        .args(common_args(current_dir).as_ref())
        .args(&[not_check_updates_flag()])
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
        .subcommand(
            SubCommand::with_name("compare")
                .setting(AppSettings::ColoredHelp)
                .visible_alias("c")
                .args(&vec![
                    Arg::with_name("input")
                        .help("Files to compare")
                        .multiple(true)
                        .min_values(2)
                        .required(true),
                    no_color_flag(),
                    quiet_flag(),
                ])
                .about("Compares if files have the same keys")
                .usage("dotenv-linter compare <files>..."),
        )
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
        no_color_flag(),
        quiet_flag(),
    ]
}

fn quiet_flag() -> clap::Arg<'static, 'static> {
    Arg::with_name("quiet")
        .short("q")
        .long("quiet")
        .help("Doesn't display additional information")
}

fn no_color_flag() -> clap::Arg<'static, 'static> {
    Arg::with_name("no-color")
        .long("no-color")
        .help("Turns off the colored output")
}

fn not_check_updates_flag() -> clap::Arg<'static, 'static> {
    Arg::with_name("not-check-updates")
        .long("not-check-updates")
        .help("Doesn't check for updates")
}
