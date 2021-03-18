use crate::common::*;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;

mod checks;
pub mod cli;
mod common;
mod fixes;
mod fs_utils;

pub use checks::available_check_names;
use common::CompareWarning;
use std::rc::Rc;

pub fn check(args: &clap::ArgMatches, current_dir: &PathBuf) -> Result<usize, Box<dyn Error>> {
    let lines_map = get_lines(args, current_dir);
    let output = CheckOutput::new(args.is_present("quiet"), lines_map.len());

    if !output.is_something_to_check() {
        return Ok(0);
    }

    let mut skip_checks: Vec<&str> = Vec::new();
    if let Some(skip) = args.values_of("skip") {
        skip_checks = skip.collect();
    }

    let warnings_count =
        lines_map
            .into_iter()
            .enumerate()
            .fold(0, |acc, (index, (fe, strings))| {
                output.print_processing_info(&fe);

                let lines = get_line_entries(&fe, strings);
                let result = checks::run(&lines, &skip_checks);

                output.print_warnings(&result, index);
                acc + result.len()
            });

    output.print_total(warnings_count);

    Ok(warnings_count)
}

pub fn fix(args: &clap::ArgMatches, current_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut warnings_count = 0;
    let lines_map = get_lines(args, current_dir);

    // Nothing to fix
    if lines_map.is_empty() {
        return Ok(());
    }

    let mut skip_checks: Vec<&str> = Vec::new();
    if let Some(skip) = args.values_of("skip") {
        skip_checks = skip.collect();
    }

    let output = FixOutput::new(args.is_present("quiet"), lines_map.len());
    for (index, (fe, strings)) in lines_map.into_iter().enumerate() {
        output.print_processing_info(&fe);

        let mut lines = get_line_entries(&fe, strings);
        let mut result = checks::run(&lines, &skip_checks);

        // run fixers & write results to file
        if !result.is_empty() && fixes::run(&mut result, &mut lines, &skip_checks) > 0 {
            let should_backup = !args.is_present("no-backup");
            // create backup copy unless user specifies not to
            if should_backup {
                let backup_file = fs_utils::backup_file(&fe)?.into_os_string();
                output.print_backup(&backup_file);
            }

            // write corrected file
            fs_utils::write_file(&fe.path, lines)?;
        }

        output.print_warnings(&result, index);
        warnings_count += result.len();
    }

    output.print_total(warnings_count);

    Ok(())
}

// Compares if different environment files contains the same variables and returns warnings if not
pub fn compare(
    args: &clap::ArgMatches,
    current_dir: &PathBuf,
) -> Result<Vec<CompareWarning>, Box<dyn Error>> {
    let mut all_keys: HashSet<String> = HashSet::new();
    let lines_map = get_lines(args, current_dir);
    let output = CompareOutput::new(args.is_present("quiet"));

    let mut warnings: Vec<CompareWarning> = Vec::new();
    let mut files_to_compare: Vec<CompareFileType> = Vec::new();

    // Nothing to check
    if lines_map.is_empty() {
        return Ok(warnings);
    }

    // Create CompareFileType structures for each file
    for (_, (fe, strings)) in lines_map.into_iter().enumerate() {
        output.print_processing_info(&fe);
        let lines = get_line_entries(&fe, strings);
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

fn get_lines(args: &clap::ArgMatches, current_dir: &PathBuf) -> BTreeMap<FileEntry, Vec<String>> {
    let file_paths: Vec<PathBuf> = get_needed_file_paths(args);

    file_paths
        .iter()
        .map(|path| fs_utils::get_relative_path(&path, &current_dir).and_then(FileEntry::from))
        .filter(Option::is_some)
        .map(Option::unwrap)
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

fn get_line_entries(fe: &FileEntry, lines: Vec<String>) -> Vec<LineEntry> {
    let fe = Rc::new(fe.clone());
    lines
        .into_iter()
        .enumerate()
        .map(|(index, line)| LineEntry::new(index + 1, fe.clone(), line))
        .collect()
}
