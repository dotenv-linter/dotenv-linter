use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::common::*;

pub use checks::available_check_names;
mod checks;
mod common;
mod fixes;
mod fs_utils;

pub mod cli;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn check(args: &clap::ArgMatches, current_dir: &Path) -> Result<usize> {
    let lines_map = get_lines(args, current_dir);
    let output = CheckOutput::new(args.is_present("quiet"), lines_map.len());

    if lines_map.is_empty() {
        output.print_nothing_to_check();
        return Ok(0);
    }

    let mut skip_checks: Vec<&str> = Vec::new();
    if let Some(skip) = args.values_of("skip") {
        skip_checks = skip.collect();
    }

    let skip_checks = skip_checks
        .into_iter()
        .filter_map(|c| LintKind::from_str(c).ok())
        .collect::<Vec<LintKind>>();

    let warnings_count =
        lines_map
            .into_iter()
            .enumerate()
            .fold(0, |acc, (index, (fe, strings))| {
                output.print_processing_info(&fe);

                let lines = get_line_entries(strings);
                let result = checks::run(&lines, &skip_checks);

                output.print_warnings(&fe, &result, index);
                acc + result.len()
            });

    output.print_total(warnings_count);

    Ok(warnings_count)
}

pub fn fix(args: &clap::ArgMatches, current_dir: &Path) -> Result<()> {
    let mut warnings_count = 0;
    let lines_map = get_lines(args, current_dir);
    let output = FixOutput::new(args.is_present("quiet"), lines_map.len());

    // Nothing to fix
    if lines_map.is_empty() {
        output.print_nothing_to_fix();
        return Ok(());
    }

    let mut skip_checks: Vec<&str> = Vec::new();
    if let Some(skip) = args.values_of("skip") {
        skip_checks = skip.collect();
    }
    let skip_checks = skip_checks
        .into_iter()
        .filter_map(|c| LintKind::from_str(c).ok())
        .collect::<Vec<LintKind>>();

    for (index, (fe, strings)) in lines_map.into_iter().enumerate() {
        output.print_processing_info(&fe);

        let mut lines = get_line_entries(strings);
        let result = checks::run(&lines, &skip_checks);
        if result.is_empty() {
            continue;
        }
        let fixes_done = fixes::run(&result, &mut lines, &skip_checks);
        if fixes_done != result.len() {
            output.print_not_all_warnings_fixed();
        }
        if fixes_done > 0 {
            let should_backup = !args.is_present("no-backup");
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
pub fn compare(args: &clap::ArgMatches, current_dir: &Path) -> Result<Vec<CompareWarning>> {
    let mut all_keys: HashSet<String> = HashSet::new();
    let lines_map = get_lines(args, current_dir);
    let output = CompareOutput::new(args.is_present("quiet"));

    let mut warnings: Vec<CompareWarning> = Vec::new();
    let mut files_to_compare: Vec<CompareFileType> = Vec::new();

    // Nothing to check
    if lines_map.is_empty() {
        output.print_nothing_to_compare();
        return Ok(warnings);
    }

    // Create CompareFileType structures for each file
    for (_, (fe, strings)) in lines_map.into_iter().enumerate() {
        output.print_processing_info(&fe);
        let lines = get_line_entries(strings);
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

fn get_lines(args: &clap::ArgMatches, current_dir: &Path) -> BTreeMap<FileEntry, Vec<String>> {
    let file_paths: Vec<PathBuf> = get_needed_file_paths(args);

    file_paths
        .iter()
        .map(|path| fs_utils::get_relative_path(path, current_dir).and_then(FileEntry::from))
        .flatten()
        .collect::<BTreeMap<_, _>>()
}

/// Getting a list of all files for checking/fixing without custom exclusion files
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

fn get_line_entries(lines: Vec<String>) -> Vec<LineEntry> {
    let length = lines.len();

    let mut lines: Vec<LineEntry> = lines
        .into_iter()
        .enumerate()
        .map(|(index, line)| LineEntry::new(index + 1, line, length == (index + 1)))
        .collect();

    // TODO: refactor
    let mut multiline_ranges: Vec<(usize, usize)> = Vec::new();
    let mut start_number: Option<usize> = None;

    // here we find ranges of multi-line values and mark them
    lines.iter_mut().for_each(|l| {
        let is_multiline_start = l
            .get_value()
            .filter(|e| e.starts_with('\'') && !e.ends_with('\''))
            .is_some();
        if start_number.is_none() && is_multiline_start {
            start_number = Some(l.number);
        } else if start_number.is_some() {
            if l.raw_string.ends_with('\'') {
                // end of multi-line value, add range to vector
                multiline_ranges.push((start_number.unwrap(), l.number))
            } else if l.get_value().is_some() {
                // if next line correct env line - then previous start-line incorrect multi-value
                start_number = None;
            }
        }
    });

    // set flags here
    for val in multiline_ranges {
        lines[val.0 - 1..val.1]
            .iter_mut()
            .for_each(|e| e.is_multiline_value = true)
    }

    lines
}
