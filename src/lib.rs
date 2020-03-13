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

fn get_args<'a>() -> clap::ArgMatches<'a> {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .version_short("v")
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

pub fn run() -> Result<Vec<Warning>, Box<dyn Error>> {
    let args = get_args();

    let dir_path = match args.value_of("path") {
        Some(path) => PathBuf::from(path),
        None => env::current_dir()?,
    };

    let files = dotenv_files(args, dir_path)?;
    let mut warnings: Vec<Warning> = Vec::new();

    for file in files {
        let f = File::open(&file.path)?;
        let reader = BufReader::new(f);

        // TODO: Initialize a vector with a capacity equal to the number of lines
        let mut lines: Vec<LineEntry> = Vec::new();
        for (index, line) in reader.lines().enumerate() {
            let number = index + 1;
            let raw_string = line?;

            lines.push(LineEntry {
                file_name: file.file_name.clone(),
                number,
                raw_string,
            })
        }

        let result = checks::run(lines);
        warnings.extend(result);
    }

    Ok(warnings)
}

/// Returns `Result<Vec<FileEntry>` of files with the the `.env` prefix
#[allow(clippy::redundant_closure)]
fn dotenv_files(
    args: clap::ArgMatches,
    dir_path: PathBuf,
) -> Result<Vec<FileEntry>, Box<dyn Error>> {
    let entries = dir_path.read_dir()?;

    let mut paths: Vec<FileEntry> = entries
        .filter_map(|e| e.ok())
        .filter_map(|f| fs::canonicalize(f.path()).ok())
        .filter_map(|p| FileEntry::from(p))
        .filter(|f| f.is_env_file())
        .collect();

    // Adds files to paths if they should be included
    if let Some(included) = args.values_of("include") {
        included
            .filter_map(|f| fs::canonicalize(f).ok())
            .filter_map(|p| FileEntry::from(p))
            .for_each(|f| paths.push(f));
    }

    // Removes files from paths if they should be excluded
    if let Some(excluded) = args.values_of("exclude") {
        let excluded_paths: Vec<PathBuf> =
            excluded.filter_map(|f| fs::canonicalize(f).ok()).collect();

        paths.retain(|f| !excluded_paths.contains(&f.path));
    }

    paths.sort();
    paths.dedup();

    Ok(paths)
}
