use crate::checks::Warning;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;
use std::process;

mod checks;

const DOTENV_PREFIX: &str = ".env";

pub struct FileEntry {
    lines: Vec<LineEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineEntry {
    number: usize,
    file_name: String,
    raw_string: String,
}

pub fn run(matches: clap::ArgMatches) -> Result<(), Error> {
    let mut paths = dotenv_files(matches)?;
    paths.dedup();

    let mut warnings: Vec<Warning> = Vec::new();
    for path in paths {
        let f = File::open(&path)?;
        let reader = BufReader::new(f);

        let file_name = match path.file_name() {
            Some(s) => s.to_str().unwrap_or("undefined").to_string(),
            None => continue,
        };

        // TODO: Initialize a vector with a capacity equal to the number of lines
        let mut lines: Vec<LineEntry> = Vec::new();
        for (index, line) in reader.lines().enumerate() {
            let number = index + 1;
            let raw_string = line?;

            lines.push(LineEntry {
                file_name: file_name.clone(),
                number,
                raw_string,
            })
        }

        let result = checks::run(FileEntry { lines });
        warnings.extend(result);
    }

    if !warnings.is_empty() {
        warnings.iter().for_each(|w| println!("{}", w));
        process::exit(1);
    }

    Ok(())
}

fn dotenv_files(matches: clap::ArgMatches) -> Result<Vec<PathBuf>, Error> {
    let current_dir = env::current_dir()?;
    let entries = current_dir.read_dir()?;

    // TODO: Use HashSet to store unique paths
    // https://doc.rust-lang.org/std/collections/struct.HashSet.html
    let mut paths: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .filter(|f| {
            f.file_name()
                .to_str()
                .filter(|s| s.starts_with(DOTENV_PREFIX))
                .is_some()
        })
        .map(|f| f.path())
        .collect();

    if let Some(includes) = matches.values_of("include") {
        let files = includes.collect::<Vec<&str>>();

        for file in files {
            // Returns the full path to the file and checks if the file exists
            if let Ok(path) = fs::canonicalize(file) {
                paths.push(path);
            }
        }
    }

    // Removes files from paths vector if they should be excluded
    if let Some(excludes) = matches.values_of("exclude") {
        let exclude_paths: Vec<PathBuf> =
            excludes.filter_map(|f| fs::canonicalize(f).ok()).collect();

        paths.retain(|p| !exclude_paths.contains(p));
    }

    Ok(paths)
}
