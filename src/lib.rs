use crate::common::*;

use std::error::Error;
use std::path::PathBuf;

mod checks;
mod common;
mod fixes;
mod fs_utils;

pub use checks::available_check_names;

#[allow(clippy::redundant_closure)]
pub fn run(args: &clap::ArgMatches, current_dir: &PathBuf) -> Result<Vec<Warning>, Box<dyn Error>> {
    let mut file_paths: Vec<PathBuf> = Vec::new();
    let mut skip_checks: Vec<&str> = Vec::new();
    let mut excluded_paths: Vec<PathBuf> = Vec::new();

    let is_recursive = args.is_present("recursive");

    if let Some(skip) = args.values_of("skip") {
        skip_checks = skip.collect();
    }

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

    let is_fix = args.is_present("fix");
    let mut warnings: Vec<Warning> = Vec::new();

    for path in file_paths {
        let relative_path = match fs_utils::get_relative_path(&path, &current_dir) {
            Some(p) => p,
            None => continue,
        };

        let (fe, strs) = match FileEntry::from(relative_path) {
            Some(f) => f,
            None => continue,
        };

        let mut lines = get_line_entries(&fe, strs);

        let mut result = checks::run(&lines, &skip_checks);
        if is_fix && fixes::run(&mut result, &mut lines) > 0 {
            fs_utils::write_file(&fe.path, lines)?;
        }

        warnings.extend(result);
    }

    Ok(warnings)
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
                .filter(|path| FileEntry::is_env_file(path) || (is_recursive && path.is_dir()))
                .collect()
        })
        .flat_map(|dir_entries| get_file_paths(dir_entries, excludes, is_recursive))
        .collect();

    let mut file_paths: Vec<PathBuf> = dir_entries
        .into_iter()
        .filter(|entry| entry.is_file())
        .filter(|entry| !excludes.contains(entry))
        .collect();

    file_paths.sort();
    file_paths.extend(nested_paths);
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
        })
    }

    entries
}
