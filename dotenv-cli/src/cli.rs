use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, command};
use dotenv_analyzer::LintKind;
use dotenv_schema::DotEnvSchema;

use crate::{CheckOptions, DiffOptions, FixOptions, Result};

const HELP_TEMPLATE: &str = "
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

#[derive(Parser)]
#[command(
    version,
    about,
    author,
    help_template = HELP_TEMPLATE,
    styles = clap::builder::Styles::styled()
        .header(clap::builder::styling::AnsiColor::Yellow.on_default())
        .usage(clap::builder::styling::AnsiColor::Yellow.on_default())
        .literal(clap::builder::styling::AnsiColor::Cyan.on_default())
        .placeholder(clap::builder::styling::AnsiColor::Blue.on_default())
        .context(clap::builder::styling::AnsiColor::Green.on_default())
)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Switch to plain text output without colors
    #[arg(long, global = true)]
    plain: bool,

    /// Display only critical results, suppressing extra details
    #[arg(short, long, global = true)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Check .env files for errors such as duplicate keys or invalid syntax
    Check {
        /// .env files or directories to check (one or more required)
        #[arg(
            num_args(1..),
            required = true,
        )]
        files: Vec<PathBuf>,

        #[command(flatten)]
        common: CommonArgs,

        /// Schema file to validate .env file contents
        #[arg(short('s'), long, value_name = "PATH")]
        schema: Option<PathBuf>,

        /// Disable checking for application updates
        #[cfg(feature = "update-informer")]
        #[arg(long, env = "DOTENV_LINTER_SKIP_UPDATES")]
        skip_updates: bool,
    },
    /// Automatically fix issues in .env files
    Fix {
        /// .env files or directories to fix (one or more required)
        #[arg(
            num_args(1..),
            required = true,
        )]
        files: Vec<PathBuf>,

        #[command(flatten)]
        common: CommonArgs,

        /// Prevent creating backups before applying fixes
        #[arg(long)]
        no_backup: bool,

        /// Print fixed .env content to stdout without saving changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Compare .env files to ensure matching key sets
    Diff {
        /// .env files or directories to compare (one or more required)
        #[arg(
            num_args(1..),
            required = true,
        )]
        files: Vec<PathBuf>,
    },
}

#[derive(Args)]
struct CommonArgs {
    /// Files or directories to exclude from linting or fixing
    #[arg(short = 'e', long, value_name = "PATH")]
    exclude: Vec<PathBuf>,

    /// Lint checks to bypass
    #[arg(
        short,
        long,
        value_name = "CHECK_NAME",
        value_delimiter = ',',
        env = "DOTENV_LINTER_IGNORE_CHECKS"
    )]
    ignore_checks: Vec<LintKind>,

    /// Recursively scan directories for .env files
    #[arg(short, long)]
    recursive: bool,
}

pub fn run() -> Result<i32> {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    let cli = Cli::parse();
    let current_dir = std::env::current_dir()?;

    if cli.plain {
        colored::control::set_override(false);
    }

    match cli.command {
        Command::Check {
            files,
            common,
            schema,
            #[cfg(feature = "update-informer")]
                skip_updates: not_check_updates,
        } => {
            let mut dotenv_schema = None;
            if let Some(path) = schema {
                dotenv_schema = match DotEnvSchema::load(path) {
                    Ok(schema) => Some(schema),
                    Err(err) => {
                        println!("Error loading schema: {err}");
                        std::process::exit(1);
                    }
                };
            }

            let total_warnings = crate::check(
                &CheckOptions {
                    files: files.iter().collect(),
                    ignore_checks: common.ignore_checks,
                    exclude: common.exclude.iter().collect(),
                    recursive: common.recursive,
                    quiet: cli.quiet,
                    schema: dotenv_schema,
                },
                &current_dir,
            )?;

            #[cfg(feature = "update-informer")]
            if !not_check_updates && !cli.quiet {
                crate::check_for_updates();
            }

            if total_warnings == 0 {
                return Ok(0);
            }
        }
        Command::Fix {
            files,
            common,
            no_backup,
            dry_run,
        } => {
            crate::fix(
                &FixOptions {
                    files: files.iter().collect(),
                    ignore_checks: common.ignore_checks,
                    exclude: common.exclude.iter().collect(),
                    recursive: common.recursive,
                    quiet: cli.quiet,

                    no_backup,
                    dry_run,
                },
                &current_dir,
            )?;

            return Ok(0);
        }
        Command::Diff { files } => {
            let total_warnings = crate::diff(
                &DiffOptions {
                    files: files.iter().collect(),
                    quiet: cli.quiet,
                },
                &current_dir,
            )?;

            if total_warnings == 0 {
                return Ok(0);
            }
        }
    }

    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
