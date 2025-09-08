use std::path::PathBuf;

use clap::{Arg, ArgAction, Command, command, value_parser};
use dotenv_analyzer::LintKind;

use self::options::{CheckOptions, CompareOptions, FixOptions};
use crate::Result;

pub mod options;

pub fn run() -> Result<i32> {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    let current_dir = std::env::current_dir()?;
    let args = command().get_matches();

    disable_color_output(&args);

    match args.subcommand() {
        None => {
            let opts = CheckOptions::new(&args);
            let total_warnings = crate::check(&opts, &current_dir)?;

            #[cfg(feature = "update-informer")]
            if !args.get_flag("not-check-updates") && !args.get_flag("quiet") {
                crate::check_for_updates();
            }

            if total_warnings == 0 {
                return Ok(0);
            }
        }
        Some(("fix", fix_args)) => {
            let opts = FixOptions::new(fix_args);
            crate::fix(&opts, &current_dir)?;

            return Ok(0);
        }
        Some(("compare", compare_args)) => {
            disable_color_output(compare_args);

            let opts = CompareOptions::new(compare_args);
            let total_warnings = crate::compare(&opts, &current_dir)?;

            if total_warnings == 0 {
                return Ok(0);
            }
        }
        _ => {
            eprintln!("unknown command");
        }
    }

    Ok(1)
}

pub fn command() -> Command {
    let mut cmd = command!()
        .disable_help_subcommand(true)
        .args(common_args())
        .subcommands([compare_command(), fix_command()]);

    if cfg!(feature = "update-informer") {
        cmd = cmd.arg(not_check_updates_flag());
    }

    cmd
}

fn compare_command() -> Command {
    Command::new("compare")
        .visible_alias("c")
        .args(vec![
            Arg::new("input")
                .help("Files to compare")
                .action(ArgAction::Append)
                .required(true)
                .num_args(2..)
                .value_parser(value_parser!(PathBuf)),
            no_color_flag(),
            quiet_flag(),
        ])
        .about("Compares if files have the same keys")
        .override_usage("dotenv-linter compare [OPTIONS] <input>...")
}

fn fix_command() -> Command {
    Command::new("fix")
        .visible_alias("f")
        .args(common_args())
        .arg(
            Arg::new("no-backup")
                .long("no-backup")
                .help("Prevents backing up .env files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("Output the fixed file to stdout without writing it to disk")
                .action(ArgAction::SetTrue),
        )
        .override_usage("dotenv-linter fix [OPTIONS] <input>...")
        .about("Automatically fixes warnings")
}

fn common_args() -> Vec<Arg> {
    vec![
        Arg::new("input")
            .help("files or paths")
            .index(1)
            .action(ArgAction::Append)
            .num_args(0..)
            .value_parser(value_parser!(PathBuf)),
        Arg::new("exclude")
            .short('e')
            .long("exclude")
            .value_name("FILE_NAME")
            .help("Excludes files from check")
            .action(ArgAction::Append)
            .num_args(0..)
            .value_parser(value_parser!(PathBuf)),
        Arg::new("skip")
            .short('s')
            .long("skip")
            .value_name("CHECK_NAME")
            .help("Skips checks")
            .action(ArgAction::Append)
            .num_args(0..)
            .value_parser(value_parser!(LintKind))
            .env("DOTENV_LINTER_SKIP")
            .value_delimiter(','),
        Arg::new("recursive")
            .short('r')
            .long("recursive")
            .help("Recursively searches and checks .env files")
            .action(ArgAction::SetTrue),
        Arg::new("schema")
            .short('S')
            .long("schema")
            .help("Use schema file to check .env files")
            .value_parser(value_parser!(PathBuf)),
        no_color_flag(),
        quiet_flag(),
    ]
}

fn quiet_flag() -> Arg {
    Arg::new("quiet")
        .short('q')
        .long("quiet")
        .help("Doesn't display additional information")
        .action(ArgAction::SetTrue)
}

fn no_color_flag() -> Arg {
    Arg::new("no-color")
        .long("no-color")
        .help("Turns off the colored output")
        .action(ArgAction::SetTrue)
}

fn not_check_updates_flag() -> Arg {
    Arg::new("not-check-updates")
        .long("not-check-updates")
        .help("Doesn't check for updates")
        .value_parser(clap::builder::BoolishValueParser::new())
        .env("DOTENV_LINTER_NOT_CHECK_UPDATES")
        .action(ArgAction::SetTrue)
}

fn disable_color_output(args: &clap::ArgMatches) {
    if args.get_flag("no-color") {
        colored::control::set_override(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        command().debug_assert();
    }
}
