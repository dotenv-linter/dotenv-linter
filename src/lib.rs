use crate::common::*;

use std::error::Error;
use std::path::PathBuf;

mod checks;
mod common;
mod fs_utils;

pub use checks::available_check_names;

#[allow(clippy::redundant_closure)]
pub fn run(args: clap::ArgMatches, current_dir: &PathBuf) -> Result<Vec<Warning>, Box<dyn Error>> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut file_paths: Vec<PathBuf> = Vec::new();
    let mut skip_checks: Vec<&str> = Vec::new();

    if let Some(skip) = args.values_of("skip") {
        skip_checks = skip.collect();
    }

    if let Some(inputs) = args.values_of("input") {
        dirs = inputs
            .clone()
            .filter_map(|s| fs_utils::canonicalize(s).ok())
            .filter(|p| p.is_dir())
            .collect();

        file_paths = inputs
            .filter_map(|s| fs_utils::canonicalize(s).ok())
            .filter(|p| p.is_file())
            .collect();
    }

    for dir_path in dirs {
        let entries = dir_path.read_dir()?;

        let mut dir_files: Vec<PathBuf> = entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|path| FileEntry::is_env_file(path))
            .collect();

        file_paths.append(dir_files.as_mut());
    }

    // Removes files from paths if they should be excluded
    if let Some(excluded) = args.values_of("exclude") {
        let excluded_paths: Vec<PathBuf> = excluded
            .filter_map(|f| fs_utils::canonicalize(f).ok())
            .collect();

        file_paths.retain(|path| !excluded_paths.contains(&path));
    }

    file_paths.sort();
    file_paths.dedup();

    let mut warnings: Vec<Warning> = Vec::new();

    for path in file_paths {
        let relative_path = match fs_utils::get_relative_path(&path, &current_dir) {
            Some(p) => p,
            None => continue,
        };

        let file_with_lines = match FileEntry::from(relative_path) {
            Some(f) => f,
            None => continue,
        };

        let lines = get_line_entries(file_with_lines.0, file_with_lines.1);

        let result = checks::run(lines, &skip_checks);
        warnings.extend(result);
    }

    Ok(warnings)
}

fn get_line_entries(fe: FileEntry, lines: Vec<String>) -> Vec<LineEntry> {
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
