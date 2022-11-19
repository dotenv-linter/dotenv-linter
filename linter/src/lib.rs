#![allow(dead_code, unused_assignments)]
use crate::cli::{Args, CompareArgs, FixArgs};
use crate::common::*;
use colored::*;
use dotenv::Files;
use std::borrow::Borrow;
use std::{collections::HashSet, path::Path};

// pub use checks::available_check_names;

mod checks;
mod common;
mod fixes;

pub mod cli;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Linter<'a> {
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

    pub fn check(&self) -> Result<usize> {
        // let dotenv_files = dotenv::_new(args.input.paths(current_dir.to_path_buf()), current_dir)
        //     .recursive(args.is_recursive())
        //     .exclude(args.exclude.paths())
        //     .lookup_files();

        let output = CheckOutput::new(self.quiet);

        if self.files.is_empty() {
            output.print_nothing_to_check();
            return Ok(0);
        }

        let output = output.files_count(self.files.count());
        // let skip_checks = args.skip.checks();

        let warnings_count =
            self.files
                .into_iter()
                .enumerate()
                .fold(0, |acc, (index, (fe, lines))| {
                    output.print_processing_info(&fe);
                    let result = checks::run(&lines, self.skip_checks);

                    output.print_warnings(&fe, &result, index);
                    acc + result.len()
                });

        output.print_total(warnings_count);
        Ok(warnings_count)
    }

    pub fn fix(&self) -> Result<()> {
        // let dotenv_files = dotenv::_new(args.input.paths(current_dir.to_path_buf()), current_dir)
        //     .recursive(args.is_recursive())
        //     .exclude(args.exclude.paths())
        //     .lookup_files();

        let output = FixOutput::new(self.quiet);

        if self.files.is_empty() {
            output.print_nothing_to_fix();
            return Ok(());
        }

        let output = output.files_count(self.files.count());
        // let skip_checks = args.skip.checks();

        let mut warnings_count = 0;
        for (index, (fe, mut lines)) in self.files.into_iter().enumerate() {
            output.print_processing_info(&fe);
            let result = checks::run(&lines, self.skip_checks);
            if result.is_empty() {
                continue;
            }
            let fixes_done = fixes::run(&result, &mut lines, self.skip_checks);
            if fixes_done != result.len() {
                output.print_not_all_warnings_fixed();
            }

            // TODO: Backup file by implementing method `backup` for FileEntry
            if fixes_done > 0 {
                // create backup copy unless user specifies not to
                if self.backup {
                    // let backup_file = fs_utils::backup_file(&fe)?;
                    // output.print_backup(&backup_file);
                }

                // write corrected file
                // fs_utils::write_file(&fe.path, lines)?;
            }

            output.print_warnings(&fe, &result, index);
            warnings_count += result.len();
        }

        output.print_total(warnings_count);
        Ok(())
    }

    // Compares if different environment files contains the same variables and returns warnings if not
    pub fn compare(&self) -> Result<Vec<CompareWarning>> {
        // let dotenv_files =
        //     dotenv::_new(args.paths(current_dir.to_path_buf()), current_dir).lookup_files();

        let output = CompareOutput::new(self.quiet);
        let mut warnings: Vec<CompareWarning> = Vec::new();

        if self.files.is_empty() {
            output.print_nothing_to_compare();
            return Ok(warnings);
        }

        // Create CompareFileType structures for each file
        let mut all_keys: HashSet<String> = HashSet::new();
        let mut files_to_compare: Vec<CompareFileType> = Vec::new();
        for (_, (fe, lines)) in self.files.into_iter().enumerate() {
            output.print_processing_info(&fe);
            let mut keys: Vec<String> = Vec::new();

            for line in lines {
                if let Some(key) = line.get_key() {
                    all_keys.insert(key.to_string());
                    keys.push(key.to_string());
                }
            }

            let file_to_compare: CompareFileType = CompareFileType {
                path: fe.path,
                keys,
                missing: Vec::new(),
            };

            files_to_compare.push(file_to_compare);
        }

        // Create warnings if any file misses any key
        for file in files_to_compare {
            let missing_keys: Vec<_> = all_keys
                .iter()
                .filter(|key| !file.keys.contains(key))
                .map(|key| key.to_owned())
                .collect();

            if !missing_keys.is_empty() {
                let warning = CompareWarning {
                    path: file.path,
                    missing_keys,
                };

                warnings.push(warning)
            }
        }

        output.print_warnings(&warnings);
        Ok(warnings)
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
