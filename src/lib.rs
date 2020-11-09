use crate::common::*;

use std::error::Error;
use std::path::PathBuf;

mod checks;
mod common;
mod fixes;
mod fs_utils;

pub use checks::available_check_names;

#[allow(clippy::redundant_closure)]
pub fn run(args: &clap::ArgMatches, current_dir: &PathBuf) -> Result<usize, Box<dyn Error>> {
    let mut warnings_count = 0;
    let file_paths: Vec<PathBuf> = get_needed_file_paths(args);

    // Nothing to check/fix
    if file_paths.is_empty() {
        return Ok(warnings_count);
    }

    let mut skip_checks: Vec<&str> = Vec::new();
    if let Some(skip) = args.values_of("skip") {
        skip_checks = skip.collect();
    }

    let is_fix = args.is_present("fix");
    let is_quiet_mode = args.is_present("quiet");

    let fix_output = FixOutput::new(is_quiet_mode);
    let check_output = CheckOutput::new(is_quiet_mode);

    for (i, path) in file_paths.iter().enumerate() {
        let relative_path = match fs_utils::get_relative_path(&path, &current_dir) {
            Some(p) => p,
            None => continue,
        };

        let (fe, strs) = match FileEntry::from(relative_path) {
            Some(f) => f,
            None => continue,
        };

        if is_fix {
            fix_output.print_processing_info(&fe);
        } else {
            check_output.print_processing_info(&fe);
        }

        let mut lines = get_line_entries(&fe, strs);

        let mut result = checks::run(&lines, &skip_checks);
        // run fixers & write results to file
        if is_fix && !result.is_empty() && fixes::run(&mut result, &mut lines, &skip_checks) > 0 {
            let should_backup = !args.is_present("no-backup");
            // create backup copy unless user specifies not to
            if should_backup {
                let backup_file = fs_utils::backup_file(&fe)?.into_os_string();
                fix_output.print_backup(&backup_file);
            }

            // write corrected file
            fs_utils::write_file(&fe.path, lines)?;
        }

        // This shouldn't be printed to Fix when combined with quiet mode
        if !(is_fix && is_quiet_mode) {
            let is_last_file = i == file_paths.len() - 1;
            check_output.print_warnings(&result, is_last_file);
        }

        warnings_count += result.len();
    }

    if is_fix {
        fix_output.print_total(warnings_count);
    } else {
        check_output.print_total(warnings_count);
    }

    Ok(warnings_count)
}

/// getting a list of all files for checking/fixing without custom exclusion files
fn get_needed_file_paths(args: &clap::ArgMatches) -> Vec<PathBuf> {
    let mut file_paths: Vec<PathBuf> = Vec::new();
    let mut excluded_paths: Vec<PathBuf> = Vec::new();

    let is_recursive = args.is_present("recursive");

    if let Some(excluded) = args.values_of("exclude") {
        excluded_paths = excluded
            .filter_map(|f| fs_utils::canonicalize(f).ok())
            .collect();
    }

    if let Some(inputs) = args.values_of("input") {
        let input_paths = inputs
            .filter_map(|s| fs_utils::canonicalize(s).ok())
            .collect();

        file_paths.extend(get_file_paths(input_paths, &excluded_paths, is_recursive));
    }

    file_paths
}

fn get_file_paths(
    dir_entries: Vec<PathBuf>,
    excludes: &[PathBuf],
    is_recursive: bool,
) -> Vec<PathBuf> {
    let nested_paths: Vec<PathBuf> = dir_entries
        .iter()
        .filter(|entry| entry.is_dir())
        .filter(|entry| !excludes.contains(entry))
        .filter_map(|dir| dir.read_dir().ok())
        .map(|read_dir| {
            read_dir
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|path| {
                    FileEntry::is_env_file(path)
                        || (is_recursive && path.is_dir() && path.read_link().is_err())
                })
                .collect()
        })
        .flat_map(|dir_entries| get_file_paths(dir_entries, excludes, is_recursive))
        .collect();

    let mut file_paths: Vec<PathBuf> = dir_entries
        .into_iter()
        .filter(|entry| entry.is_file())
        .filter(|entry| !excludes.contains(entry))
        .collect();

    file_paths.extend(nested_paths);
    file_paths.sort();
    file_paths.dedup();
    file_paths
}

fn get_line_entries(fe: &FileEntry, lines: Vec<String>) -> Vec<LineEntry> {
    let mut entries: Vec<LineEntry> = Vec::with_capacity(fe.total_lines);

    for (index, line) in lines.into_iter().enumerate() {
        entries.push(LineEntry {
            number: index + 1,
            file: fe.clone(),
            raw_string: line,
            is_deleted: false,
        })
    }

    entries
}
