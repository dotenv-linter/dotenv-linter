use crate::Result;
use clap::{command, Arg, ArgAction, Command};

pub fn run() -> Result<i32> {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    let current_dir = std::env::current_dir()?;
    let args = command().get_matches();

    disable_color_output(&args);

    match args.subcommand() {
        None => {
            let total_warnings = crate::check(&args, &current_dir)?;

            if total_warnings == 0 {
                return Ok(0);
            }
        }
        Some(("fix", fix_args)) => {
            crate::fix(fix_args, &current_dir)?;

            return Ok(0);
        }
        Some(("list", _)) => {
            crate::available_check_names()
                .iter()
                .for_each(|name| println!("{}", name));

            return Ok(0);
        }
        Some(("compare", compare_args)) => {
            disable_color_output(compare_args);

            let warnings = crate::compare(compare_args, &current_dir)?;
            if warnings.is_empty() {
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
    command!()
        .disable_help_subcommand(true)
        .args(common_args())
        .arg(not_check_updates_flag())
        .subcommands([compare_command(), fix_command(), list_command()])
}

fn compare_command() -> Command {
    Command::new("compare")
        .visible_alias("c")
        .args(&vec![
            Arg::new("input")
                .help("Files to compare")
                .action(ArgAction::Append)
                .required(true)
                .num_args(2..),
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
        .override_usage("dotenv-linter fix [OPTIONS] <input>...")
        .about("Automatically fixes warnings")
}

fn list_command() -> Command {
    Command::new("list")
        .visible_alias("l")
        .override_usage("dotenv-linter list")
        .about("Shows list of available checks")
}

fn common_args() -> Vec<Arg> {
    vec![
        Arg::new("input")
            .help("files or paths")
            .index(1)
            .action(ArgAction::Append)
            .num_args(0..),
        Arg::new("exclude")
            .short('e')
            .long("exclude")
            .value_name("FILE_NAME")
            .help("Excludes files from check")
            .action(ArgAction::Append)
            .num_args(0..),
        Arg::new("skip")
            .short('s')
            .long("skip")
            .value_name("CHECK_NAME")
            .help("Skips checks")
            .action(ArgAction::Append)
            .num_args(0..),
        Arg::new("recursive")
            .short('r')
            .long("recursive")
            .help("Recursively searches and checks .env files")
            .action(ArgAction::SetTrue),
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
