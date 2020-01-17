use crate::common::*;
use clap::Arg;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;

mod checks;
mod common;

const DOTENV_PREFIX: &str = ".env";

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

        let file_paths = self.dotenv_files(dir_path)?;

        let mut warnings: Vec<Warning> = Vec::new();
        for path in file_paths {
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

        Ok(warnings)
    }

    fn dotenv_files(&self, dir_path: PathBuf) -> Result<HashSet<PathBuf>, Error> {
        let entries = dir_path.read_dir()?;

        let mut paths: HashSet<PathBuf> = entries
            .filter_map(Result::ok)
            .filter(|f| {
                f.file_name()
                    .to_str()
                    .filter(|s| s.starts_with(DOTENV_PREFIX))
                    .is_some()
            })
            .map(|f| f.path())
            .collect();

        if let Some(includes) = self.args.values_of("include") {
            let files = includes.collect::<Vec<&str>>();

            for file in files {
                // Returns the full path to the file and checks if the file exists
                if let Ok(path) = fs::canonicalize(file) {
                    paths.insert(path);
                }
            }
        }

        // Removes files from paths vector if they should be excluded
        if let Some(excludes) = self.args.values_of("exclude") {
            let exclude_paths: Vec<PathBuf> =
                excludes.filter_map(|f| fs::canonicalize(f).ok()).collect();

            paths.retain(|p| !exclude_paths.contains(p));
        }

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
