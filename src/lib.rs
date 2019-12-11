use std::env;
use std::fmt;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader, Error};

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

pub fn run() -> Result<(), Error> {
    let files = dotenv_files()?;

    for file in files {
        let f = File::open(file.path())?;
        let reader = BufReader::new(f);
        let file_name = match file.file_name().to_str() {
            Some(s) => s.to_string(),
            None => continue,
        };

        for (index, line) in reader.lines().enumerate() {
            let raw_string = line?;
            let number = index + 1;

            checks::run(&LineEntry { number, raw_string })
                .iter()
                .for_each(|w| println!("{}:{} {}", file_name, number, w));
        }
    }

    Ok(())
}

fn dotenv_files() -> Result<Vec<DirEntry>, Error> {
    let current_dir = env::current_dir()?;
    let entries = current_dir.read_dir()?;

    let files = entries
        .filter_map(Result::ok)
        .filter(|f| {
            f.file_name()
                .to_str()
                .filter(|s| s.starts_with(DOTENV_PREFIX))
                .is_some()
        })
        .collect::<Vec<DirEntry>>();

    Ok(files)
}
