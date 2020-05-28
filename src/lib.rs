use crate::common::*;

use clap::Arg;
use std::error::Error;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{env, fs, io, process};

mod checks;
mod common;

fn get_args(current_dir: &OsStr) -> clap::ArgMatches {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .version_short("v")
        .arg(
            Arg::with_name("input")
                .help("files or paths")
                .index(1)
                .default_value_os(current_dir)
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("exclude")
                .short("e")
                .long("exclude")
                .value_name("FILE_NAME")
                .help("Excludes files from check")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("skip")
                .short("s")
                .long("skip")
                .value_name("CHECK_NAME")
                .help("Skips checks")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("show-checks")
                .long("show-checks")
                .help("Shows list of available checks"),
        )
        .get_matches()
}

#[allow(clippy::redundant_closure)]
pub fn run() -> Result<Vec<Warning>, Box<dyn Error>> {
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => return Err(Box::new(e)),
    };

    let current_dir = current_dir.as_os_str();

    let args = get_args(current_dir);

    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut file_paths: Vec<PathBuf> = Vec::new();
    let mut skip_checks: Vec<&str> = Vec::new();

    if args.is_present("show-checks") {
        checks::available_check_names()
            .iter()
            .for_each(|name| println!("{}", name));
        process::exit(0);
    }

    if let Some(skip) = args.values_of("skip") {
        skip_checks = skip.collect();
    }

    if let Some(inputs) = args.values_of("input") {
        dirs = inputs
            .clone()
            .filter_map(|s| fs::canonicalize(s).ok())
            .filter(|p| p.is_dir())
            .collect();

        file_paths = inputs
            .filter_map(|s| fs::canonicalize(s).ok())
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
        let excluded_paths: Vec<PathBuf> =
            excluded.filter_map(|f| fs::canonicalize(f).ok()).collect();

        file_paths.retain(|path| !excluded_paths.contains(&path));
    }

    file_paths.sort();
    file_paths.dedup();

    let mut warnings: Vec<Warning> = Vec::new();

    for path in file_paths {
        let strip_path = match path.strip_prefix(&current_dir) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let file_with_lines = match FileEntry::from(strip_path.to_owned()) {
            Some(f) => f,
            _ => continue,
        };

        let lines = get_line_entries(file_with_lines.0, file_with_lines.1)?;

        let result = checks::run(lines, &skip_checks);
        warnings.extend(result);
    }

    Ok(warnings)
}

fn get_line_entries(fe: FileEntry, lines: Vec<String>) -> io::Result<Vec<LineEntry>> {
    let mut entries: Vec<LineEntry> = Vec::with_capacity(fe.total_lines);

    for (index, line) in lines.into_iter().enumerate() {
        entries.push(LineEntry {
            number: index + 1,
            file: fe.clone(),
            raw_string: line,
        })
    }

    Ok(entries)
}
