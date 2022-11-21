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
// #[clap(disable_help_subcommand = true)]
#[clap(disable_version_flag = true)]
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
    #[clap(num_args(2..))]
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
