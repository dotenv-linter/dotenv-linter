use std::env;
use std::error::Error;
use std::fs::{File, DirEntry};
use std::io::{BufRead, BufReader};

const DOTENV_PREFIX: &str = ".env";

pub fn run() -> Result<(), Box<dyn Error>> {
    let files = dotenv_files()?;

    for file in files {
        let f = File::open(file.path())?;
        let reader = BufReader::new(f);

        for (_index, _line) in reader.lines().enumerate() {
            // Run checks here...
        }
    }

    Ok(())
}

fn dotenv_files() -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let entries = current_dir.read_dir()?;
    let mut dotenv_files: Vec<DirEntry> = Vec::new();

    entries
        .filter_map(Result::ok)
        .filter(|f| {
            // TODO: Get rid of unwrap()
            f.file_name().to_str().unwrap().starts_with(DOTENV_PREFIX)
        })
        .for_each(|f| dotenv_files.push(f));

    Ok(dotenv_files)
}
