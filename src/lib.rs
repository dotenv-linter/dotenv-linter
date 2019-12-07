use std::env;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader, Error};

const DOTENV_PREFIX: &str = ".env";

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
            // Run checks here...

            let line = line?;
            if let Err(e) = check_leading_space(&line) {
                println!("{}:{} {}", file_name, index + 1, e);
            }
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


fn check_leading_space(line: &String) -> Result<(), String> {
    if line.starts_with(' ') {
        Err(String::from("Leading space detected"))
    } else {
        Ok(())
    }
}
