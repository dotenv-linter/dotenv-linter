use crate::common::*;
use clap::Arg;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;

mod checks;
mod common;

fn new_app<'a, 'b>() -> clap::App<'a, 'b> {
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
}

#[derive(Debug)]
pub struct DotenvLinter<'a> {
    args: clap::ArgMatches<'a>,
}

pub fn new<'a>() -> DotenvLinter<'a> {
    DotenvLinter {
        args: new_app().get_matches(),
    }
}

impl<'a> DotenvLinter<'a> {
    pub fn run(&self) -> Result<Vec<Warning>, Error> {
        let dir_path = match self.args.value_of("path") {
            Some(path) => PathBuf::from(path),
            None => env::current_dir()?,
        };

        let files = self.dotenv_files(dir_path)?;

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
    fn dotenv_files(&self, dir_path: PathBuf) -> Result<Vec<FileEntry>, Error> {
        let entries = dir_path.read_dir()?;

        let mut paths: Vec<FileEntry> = entries
            .filter_map(Result::ok)
            .filter_map(|e| FileEntry::from(e.path()))
            .filter(|f| f.is_env_file())
            .collect();

        // Adds files to paths if they should be included
        if let Some(included) = self.args.values_of("include") {
            included
                .filter_map(|f| fs::canonicalize(f).ok())
                .filter_map(|p| FileEntry::from(p))
                .for_each(|f| paths.push(f));
        }

        // Removes files from paths if they should be excluded
        if let Some(excluded) = self.args.values_of("exclude") {
            let excluded_paths: Vec<PathBuf> =
                excluded.filter_map(|f| fs::canonicalize(f).ok()).collect();

            paths.retain(|f| !excluded_paths.contains(&f.path));
        }

        paths.sort();
        paths.dedup();

        Ok(paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod args {
        use super::*;

        mod path {
            use super::*;
            use std::io::ErrorKind;

            #[test]
            fn file_not_found() {
                let linter = DotenvLinter {
                    args: new_app().get_matches_from(vec!["dotenv-linter", "-p", "/foo"]),
                };

                let result = linter.run();
                assert!(result.is_err());
                // Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
                assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
            }
        }
    }
}
