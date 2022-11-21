use crate::{checks, LintKind, Result};
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

// TODO https://github.com/ducaale/xh/blob/master/src/cli.rs

// TODO: Look at
// https://docs.rs/clap_complete/latest/clap_complete

// TODO: remove debug
#[derive(clap::Parser, Debug)]
#[clap(about, author, version)]
#[clap(disable_help_subcommand = true)]
pub(crate) struct Args {
    /// Exclude files from check
    #[clap(short, long)]
    pub(crate) exclude: Vec<PathBuf>,

    // TODO: Use ^ PathBuf?
    /// Files or paths
    pub(crate) input: Vec<String>,

    /// Turn off the colored output
    #[clap(long)]
    no_color: bool,

    /// Don't check for updates
    #[clap(long)]
    pub(crate) not_check_updates: bool,

    /// Don't display additional information
    #[clap(short, long)]
    pub(crate) quiet: bool,

    /// Recursively search and check .env files
    #[clap(short, long)]
    pub(crate) recursive: bool,

    /// Skip checks
    #[clap(short, long)]
    pub(crate) skip: Vec<String>,

    /// Print version information
    #[clap(short, long)]
    version: bool,

    #[clap(subcommand)]
    command: Option<Commands>,
}

// impl Skip {
//     pub(crate) fn checks(&self) -> Vec<LintKind> {
//         self.skip
//             .iter()
//             .filter_map(|check| LintKind::from_str(check).ok())
//             .collect()
//     }
// }

// TODO: remove debug
// TODO: Add tests for aliases
#[derive(Debug, clap::Subcommand)]
enum Commands {
    /// Compare if files have the same keys
    #[clap(visible_alias = "c")]
    Compare(CompareArgs),

    /// Automatically fix warnings
    #[clap(visible_alias = "f")]
    Fix(FixArgs),

    /// Show list of available checks
    #[clap(visible_alias = "l")]
    List,
}

// TODO: remove debug
#[derive(clap::Args, Debug)]
pub(crate) struct CompareArgs {
    /// Files or paths
    #[clap(min_values = 2)]
    pub(crate) input: Vec<String>,

    /// Turn off the colored output
    #[clap(long)]
    no_color: bool,

    /// Don't display additional information
    #[clap(short, long)]
    pub(crate) quiet: bool,
}

// impl CompareArgs {
// pub(crate) fn is_quiet(&self) -> bool {
//     self.quiet.quiet
// }

// pub(crate) fn paths(&self, current_dir: PathBuf) -> Vec<PathBuf> {
//     let mut input: Vec<PathBuf> = self
//         .input
//         .iter()
//         .filter_map(|f| fs_utils::canonicalize(f).ok())
//         .collect();
//
//     if input.is_empty() {
//         input.push(current_dir);
//     }
//
//     input
// }
// }

// TODO: remove debug
#[derive(clap::Args, Debug)]
pub(crate) struct FixArgs {
    /// Exclude files from check
    #[clap(short, long)]
    pub(crate) exclude: Vec<PathBuf>,

    /// Prevent backing up .env files
    #[clap(long)]
    no_backup: bool,

    /// Files or paths
    pub(crate) input: Vec<String>,

    /// Turn off the colored output
    #[clap(long)]
    no_color: bool,

    /// Don't display additional information
    #[clap(short, long)]
    pub(crate) quiet: bool,

    /// Recursively search and check .env files
    #[clap(short, long)]
    pub(crate) recursive: bool,

    /// Skip checks
    #[clap(short, long)]
    pub(crate) skip: Vec<String>,
}

pub fn run() -> Result<i32> {
    let current_dir = env::current_dir()?;
    let dotenv = dotenv::new(current_dir);
    let args: Args = Args::parse();

    match args.command {
        Some(Commands::Compare(args)) => {
            println!("compare...");
            dbg!(&args);

            let files = dotenv.input(args.input).lookup_files();

            let warnings = crate::new(files, args.quiet).compare()?;
            if warnings == 0 {
                return Ok(0);
            }
        }
        Some(Commands::Fix(args)) => {
            println!("fix...");
            dbg!(&args);

            let files = dotenv
                .input(args.input)
                .recursive(args.recursive)
                .exclude(args.exclude)
                .lookup_files();
            let skip_checks = args
                .skip
                .iter()
                .filter_map(|check| LintKind::from_str(check).ok())
                .collect::<Vec<LintKind>>();

            crate::new(files, args.quiet)
                .skip_checks(&skip_checks)
                .backup(!args.no_backup)
                .fix()?;

            return Ok(0);
        }
        Some(Commands::List) => {
            println!("list...");

            checks::available_check_names()
                .iter()
                .for_each(|name| println!("{}", name));
            return Ok(0);
        }
        None => {
            println!("check...");

            let files = dotenv.input(args.input).lookup_files();
            let skip_checks = args
                .skip
                .iter()
                .filter_map(|check| LintKind::from_str(check).ok())
                .collect::<Vec<LintKind>>();

            let total_warnings = crate::new(files, args.quiet)
                .skip_checks(&skip_checks)
                .check()?;

            if !args.not_check_updates && !args.quiet {
                crate::print_new_version_if_available();
            }

            if total_warnings == 0 {
                return Ok(0);
            }
        }
    }

    Ok(1)
}

// pub fn new2(current_dir: &OsStr) -> App {
//     App::new(env!("CARGO_PKG_NAME"))
//        .setting(AppSettings::DisableHelpSubcommand)
//         .about(env!("CARGO_PKG_DESCRIPTION"))
//         .author(env!("CARGO_PKG_AUTHORS"))
//         .version(env!("CARGO_PKG_VERSION"))
//         .mut_arg("version", |a| a.short('v'))
//         .args(common_args(current_dir))
//         .subcommand(
//             App::new("list")
//                 .visible_alias("l")
//                 .override_usage("dotenv-linter list")
//                 .about("Shows list of available checks"),
//         )
//         .subcommand(
//             App::new("fix")
//                 .visible_alias("f")
//                 .args(common_args(current_dir))
//                 .arg(
//                     Arg::new("no-backup")
//                         .long("no-backup")
//                         .help("Prevents backing up .env files"),
//                 )
//                 .override_usage("dotenv-linter fix [FLAGS] [OPTIONS] <input>...")
//                 .about("Automatically fixes warnings"),
//         )
//         .subcommand(
//             App::new("compare")
//                 .visible_alias("c")
//                 .args(&vec![
//                     Arg::new("input")
//                         .help("Files to compare")
//                         .multiple(true)
//                         .min_values(2)
//                         .required(true),
//                     no_color_flag(),
//                     quiet_flag(),
//                 ])
//                 .about("Compares if files have the same keys")
//                 .override_usage("dotenv-linter compare <files>..."),
//         )
// }
//
// fn common_args(current_dir: &OsStr) -> Vec<Arg> {
//     vec![
//         Arg::new("input")
//             .help("files or paths")
//             .index(1)
//             .default_value_os(current_dir)
//             .required(true)
//             .multiple(true),
//         Arg::new("exclude")
//             .short('e')
//             .long("exclude")
//             .value_name("FILE_NAME")
//             .help("Excludes files from check")
//             .multiple(true)
//             .takes_value(true),
//         Arg::new("skip")
//             .short('s')
//             .long("skip")
//             .value_name("CHECK_NAME")
//             .help("Skips checks")
//             .multiple(true)
//             .takes_value(true),
//         Arg::new("recursive")
//             .short('r')
//             .long("recursive")
//             .help("Recursively searches and checks .env files"),
//         no_color_flag(),
//         quiet_flag(),
//     ]
// }
//
// fn quiet_flag<'a>() -> clap::Arg<'a> {
//     Arg::new("quiet")
//         .short('q')
//         .long("quiet")
//         .help("Doesn't display additional information")
// }
//
// fn no_color_flag<'a>() -> clap::Arg<'a> {
//     Arg::new("no-color")
//         .long("no-color")
//         .help("Turns off the colored output")
// }
//
// fn not_check_updates_flag<'a>() -> clap::Arg<'a> {
//     Arg::with_name("not-check-updates")
//         .long("not-check-updates")
//         .help("Doesn't check for updates")
// }

// Newest

// use clap::{Arg, Command};
// use std::ffi::OsStr;
//
// pub fn new(current_dir: &OsStr) -> Command {
//     Command::new(env!("CARGO_PKG_NAME"))
//         .about(env!("CARGO_PKG_DESCRIPTION"))
//         .author(env!("CARGO_PKG_AUTHORS"))
//         .version(env!("CARGO_PKG_VERSION"))
//         .disable_help_subcommand(true)
//         .propagate_version(true)
//         .mut_arg("version", |a| a.short('v'))
//         .args(common_args(current_dir))
//         .arg(not_check_updates_flag())
//         .subcommands([compare_command(), fix_command(current_dir), list_command()])
// }
//
// fn compare_command<'a>() -> Command<'a> {
//     Command::new("compare")
//         .visible_alias("c")
//         .args(&vec![
//             Arg::new("input")
//                 .help("Files to compare")
//                 .multiple_occurrences(true)
//                 .multiple_values(true)
//                 .min_values(2)
//                 .required(true),
//             no_color_flag(),
//             quiet_flag(),
//         ])
//         .about("Compares if files have the same keys")
//         .override_usage("dotenv-linter compare [OPTIONS] <input>...")
// }
//
// fn fix_command(current_dir: &OsStr) -> Command {
//     Command::new("fix")
//         .visible_alias("f")
//         .args(common_args(current_dir))
//         .arg(
//             Arg::new("no-backup")
//                 .long("no-backup")
//                 .help("Prevents backing up .env files"),
//         )
//         .override_usage("dotenv-linter fix [OPTIONS] <input>...")
//         .about("Automatically fixes warnings")
// }
//
// fn list_command<'a>() -> Command<'a> {
//     Command::new("list")
//         .visible_alias("l")
//         .override_usage("dotenv-linter list")
//         .about("Shows list of available checks")
// }
//
// fn common_args(current_dir: &OsStr) -> Vec<Arg> {
//     vec![
//         Arg::new("input")
//             .help("files or paths")
//             .index(1)
//             .default_value_os(current_dir)
//             .multiple_occurrences(true)
//             .multiple_values(true),
//         Arg::new("exclude")
//             .short('e')
//             .long("exclude")
//             .value_name("FILE_NAME")
//             .help("Excludes files from check")
//             .multiple_occurrences(true)
//             .multiple_values(true)
//             .takes_value(true),
//         Arg::new("skip")
//             .short('s')
//             .long("skip")
//             .value_name("CHECK_NAME")
//             .help("Skips checks")
//             .multiple_occurrences(true)
//             .multiple_values(true)
//             .takes_value(true),
//         Arg::new("recursive")
//             .short('r')
//             .long("recursive")
//             .help("Recursively searches and checks .env files"),
//         no_color_flag(),
//         quiet_flag(),
//     ]
// }
//
// fn quiet_flag<'a>() -> Arg<'a> {
//     Arg::new("quiet")
//         .short('q')
//         .long("quiet")
//         .help("Doesn't display additional information")
// }
//
// fn no_color_flag<'a>() -> Arg<'a> {
//     Arg::new("no-color")
//         .long("no-color")
//         .help("Turns off the colored output")
// }
//
// fn not_check_updates_flag<'a>() -> Arg<'a> {
//     Arg::new("not-check-updates")
//         .long("not-check-updates")
//         .help("Doesn't check for updates")
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::env;
//
//     #[test]
//     fn verify_app() {
//         let current_dir = env::current_dir().expect("Failed to get current dir");
//         new(current_dir.as_os_str()).debug_assert();
//     }
// }
