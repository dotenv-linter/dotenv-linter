use crate::common::*;

use clap::Arg;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod checks;
mod common;

fn get_args(current_dir: &str) -> clap::ArgMatches {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .version_short("v")
        .arg(
            Arg::with_name("input")
                .help("files or paths")
                .index(1)
                .default_value(current_dir)
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("include")
                .short("i")
                .long("include")
                .value_name("FILE_NAME")
                .help("Includes files to check")
                .multiple(true)
                .takes_value(true),
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
            Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("DIRECTORY_PATH")
                .help("Specify the path of the directory where to run dotenv-linter")
                .multiple(false)
                .takes_value(true),
        )
        .get_matches()
}

#[allow(clippy::redundant_closure)]
pub fn run() -> Result<Vec<Warning>, Box<dyn Error>> {
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => return Err(Box::new(e)),
    };

    let current_dir = current_dir.to_str().unwrap();

    let args = get_args(current_dir);

    let mut paths: Vec<PathBuf> = vec![];
    let mut files: Vec<FileEntry> = vec![];

    if let Some(inputs) = args.values_of("input") {
        paths = inputs
            .clone()
            .filter_map(|s| fs::canonicalize(s).ok())
            .filter(|p| p.is_dir())
            .collect();

        files = inputs
            .filter_map(|s| fs::canonicalize(s).ok())
            .filter(|p| p.is_file())
            .filter_map(|p| FileEntry::from(p))
            .collect();
    }

    for path in paths {
        let entries = path.read_dir()?;

        let mut file_paths: Vec<FileEntry> = entries
            .filter_map(|e| e.ok())
            .filter_map(|e| FileEntry::from(e.path()))
            .filter(|f| f.is_env_file())
            .collect();

        files.append(file_paths.as_mut());
    }

    // Removes files from paths if they should be excluded
    if let Some(excluded) = args.values_of("exclude") {
        let excluded_paths: Vec<PathBuf> =
            excluded.filter_map(|f| fs::canonicalize(f).ok()).collect();

        files.retain(|f| !excluded_paths.contains(&f.path));
    }

    files.sort();
    files.dedup();

    let mut new_files: Vec<FileEntry> = vec![];
    for file in files {
        let result = file.path.strip_prefix(&current_dir);
        let strip_path = match result {
            Ok(p) => p,
            Err(_) => continue,
        };

        let new_file = match FileEntry::from(strip_path.to_owned()) {
            Some(f) => f,
            None => continue,
        };

        new_files.push(new_file);
    }

    let mut warnings: Vec<Warning> = Vec::new();
    for file in new_files {
        let f = File::open(&file.path)?;
        let reader = BufReader::new(f);

        // TODO: Initialize a vector with a capacity equal to the number of lines
        let mut lines: Vec<LineEntry> = Vec::new();
        for (index, line) in reader.lines().enumerate() {
            let number = index + 1;
            let raw_string = line?;

            lines.push(LineEntry {
                number,
                file_path: file.path.clone(),
                raw_string,
            })
        }

        let result = checks::run(lines);
        warnings.extend(result);
    }

    Ok(warnings)
}
