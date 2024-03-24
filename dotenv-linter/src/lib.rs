use crate::{
    cli::options::{CheckOptions, CompareOptions, FixOptions},
    common::*,
};
use std::{collections::HashSet, path::PathBuf};

mod checks;
mod common;
mod fixes;
mod fs_utils;

pub mod cli;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn check(opts: &CheckOptions, current_dir: &PathBuf) -> Result<usize> {
    let files = dotenv_lookup::new(current_dir, &opts.input)
        .recursive(opts.recursive)
        .exclude(&opts.exclude)
        .lookup_files();

    let output = CheckOutput::new(opts.quiet);

    if files.is_empty() {
        output.print_nothing_to_check();
        return Ok(0);
    }

    let output = output.files_count(files.len());

    let warnings_count = files
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, (fe, lines))| {
            output.print_processing_info(&fe);
            let result = checks::run(&lines, &opts.skip);

            output.print_warnings(&fe, &result, index);
            acc + result.len()
        });

    output.print_total(warnings_count);
    Ok(warnings_count)
}

pub fn fix(opts: &FixOptions, current_dir: &PathBuf) -> Result<()> {
    let files = dotenv_lookup::new(current_dir, &opts.input)
        .recursive(opts.recursive)
        .exclude(&opts.exclude)
        .lookup_files();

    let output = FixOutput::new(opts.quiet);

    if files.is_empty() {
        output.print_nothing_to_fix();
        return Ok(());
    }

    let output = output.files_count(files.len());

    let mut warnings_count = 0;
    for (index, (fe, mut lines)) in files.into_iter().enumerate() {
        output.print_processing_info(&fe);

        // let mut lines = get_line_entries(strings);
        let result = checks::run(&lines, &opts.skip);
        if result.is_empty() {
            continue;
        }
        let fixes_done = fixes::run(&result, &mut lines, &opts.skip);
        if fixes_done != result.len() {
            output.print_not_all_warnings_fixed();
        }
        if fixes_done > 0 {
            let should_backup = !opts.no_backup;
            // create backup copy unless user specifies not to
            if should_backup {
                let backup_file = fs_utils::backup_file(&fe)?;
                output.print_backup(&backup_file);
            }

            // write corrected file
            fs_utils::write_file(&fe.path, lines)?;
        }

        output.print_warnings(&fe, &result, index);
        warnings_count += result.len();
    }

    output.print_total(warnings_count);
    Ok(())
}

// Compares if different environment files contains the same variables and returns warnings if not
pub fn compare(opts: &CompareOptions, current_dir: &PathBuf) -> Result<usize> {
    let files = dotenv_lookup::new(current_dir, &opts.input).lookup_files();
    let output = CompareOutput::new(opts.quiet);

    if files.is_empty() {
        output.print_nothing_to_compare();
        return Ok(0);
    }

    // Create CompareFileType structures for each file
    let mut all_keys: HashSet<String> = HashSet::new();
    let mut files_to_compare: Vec<CompareFileType> = Vec::new();
    for (fe, lines) in files.into_iter() {
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
    let mut warnings: Vec<CompareWarning> = Vec::new();
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

    // Create success message if no warnings found.
    if warnings.is_empty() {
        output.print_no_difference_found();
        return Ok(0);
    }

    output.print_warnings(&warnings);
    Ok(warnings.len())
}

/// Checks for updates and prints information about the new version to `STDOUT`
#[cfg(feature = "update-informer")]
pub(crate) fn check_for_updates() {
    use colored::*;
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
