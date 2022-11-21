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
