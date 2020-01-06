use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;

mod checks;

const DOTENV_PREFIX: &str = ".env";

pub struct LineEntry {
    number: usize,
    raw_string: String,
}

impl fmt::Display for LineEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "number: {}, raw_string: {}",
            self.number, self.raw_string
        )
    }
}

pub fn run(matches: clap::ArgMatches) -> Result<(), Error> {
    let paths = dotenv_files(matches)?;

    for path in paths {
        let f = File::open(&path)?;
        let reader = BufReader::new(f);

        let file_name = match path.file_name() {
            Some(s) => s.to_str().unwrap_or("undefined"),
            None => continue,
        };

        for (index, line) in reader.lines().enumerate() {
            let raw_string = line?;

            // A comment or empty line should just be skipped
            let trimmed_string = raw_string.trim();
            if trimmed_string.starts_with('#') || trimmed_string.is_empty() {
                continue;
            }

            let number = index + 1;

            checks::run(&LineEntry { number, raw_string })
                .iter()
                .for_each(|w| println!("{}:{} {}", file_name, number, w));
        }
    }

    Ok(())
}

fn dotenv_files(matches: clap::ArgMatches) -> Result<Vec<PathBuf>, Error> {
    let current_dir = env::current_dir()?;
    let entries = current_dir.read_dir()?;

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
        let files = excludes.collect::<Vec<&str>>();

        for file in files {
            if let Ok(file) = fs::canonicalize(file) {
                if paths.contains(&file) {
                    for (index, path) in paths.clone().iter().enumerate() {
                        if path == &file {
                            paths.remove(index);
                        }
                    }
                }
            }
        }
    }

    Ok(paths)
}
