use std::env;
use std::error::Error;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader};

const DOTENV_PREFIX: &str = ".env";
const LEADING_SPACE_WARNING: &str = "Leading space detected";

pub fn run() -> Result<(), Box<dyn Error>> {
    let files = dotenv_files()?;

    for file in files {
        let f = File::open(file.path())?;
        let reader = BufReader::new(f);

        for (index, line) in reader.lines().enumerate() {
            if let Err(e) = check_leading_space(line?) {
                println!(
                    "{}:{} {}",
                    // TODO: Get rid of unwrap()
                    file.file_name().to_str().unwrap(),
                    index + 1,
                    e
                );
            }
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

fn check_leading_space(line: String) -> Result<(), &'static str> {
    if line.starts_with(" ") {
        return Err(LEADING_SPACE_WARNING);
    } else {
        Ok(())
    }
}
