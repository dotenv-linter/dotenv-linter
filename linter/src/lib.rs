// #![allow(dead_code, unused_assignments)]
use crate::command::{check, compare, fix};
use crate::common::*;
use colored::*;
use dotenv::Files;

// pub use checks::available_check_names;

mod checks;
mod common;
mod fixes;

pub mod cli;
mod command;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Linter<'a> {
    files: Files,
    quiet: bool,
    backup: bool,
    skip_checks: &'a [LintKind],
}

pub fn new<'a>(files: Files, quiet: bool) -> Linter<'a> {
    Linter {
        files,
        quiet,
        skip_checks: &[],
        backup: false,
    }
}

impl<'a> Linter<'a> {
    pub fn backup(self, backup: bool) -> Self {
        Self { backup, ..self }
    }

    pub fn skip_checks(self, skip_checks: &'a [LintKind]) -> Self {
        Self {
            skip_checks,
            ..self
        }
    }

    pub fn check(self) -> Result<usize> {
        command::check(self)
    }

    pub fn fix(self) -> Result<usize> {
        command::fix(self)
    }

    pub fn compare(self) -> Result<usize> {
        command::compare(self)
    }

    pub fn list(self) -> Vec<LintKind> {
        command::list()
    }
}

// Crates:
// dotenv - finds .env files
// linter - checks and fixes .env files

// Parts:
// main -> cli (parse args) -> dotenv -> linter -> printer

// TODO: Move commands to modules

// TODO: Pass current_dir once
// TODO: Commands must return result, not print data via output

/// Prints information about the new version to `STDOUT` if a new version is available
pub(crate) fn print_new_version_if_available() {
    use update_informer::{registry, Check};

    let pkg_name = env!("CARGO_PKG_NAME");

    #[cfg(not(feature = "stub_check_version"))]
    let current_version = env!("CARGO_PKG_VERSION");
    #[cfg(feature = "stub_check_version")]
    let current_version = "3.0.0";

    #[cfg(not(feature = "stub_check_version"))]
    let informer = update_informer::new(registry::Crates, pkg_name, current_version);
    #[cfg(feature = "stub_check_version")]
    let informer = update_informer::fake(registry::Crates, pkg_name, current_version, "3.1.1");

    if let Ok(Some(version)) = informer.check_version() {
        let msg = format!(
            "A new release of {pkg_name} is available: v{current_version} -> {new_version}",
            pkg_name = pkg_name.italic().cyan(),
            current_version = current_version,
            new_version = version.to_string().green()
        );

        let release_url = format!(
            "https://github.com/{pkg_name}/{pkg_name}/releases/tag/{version}",
            pkg_name = pkg_name,
            version = version
        )
        .yellow();

        println!("\n{msg}\n{url}", msg = msg, url = release_url);
    }
}
