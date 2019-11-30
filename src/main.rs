use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let entries = current_dir.read_dir().unwrap();
    let mut dotenv_files = Vec::new();

    entries
        .filter_map(Result::ok)
        .filter(|f| f.path().file_name().unwrap().to_str().unwrap().starts_with(".env") )
        .for_each(|f| dotenv_files.push(f));

    let regexp = Regex::new(r"^(\S*)\s*=\s*(\S*)$").unwrap();

    for file in dotenv_files {
        let f = File::open(file.path()).unwrap();
        let f = BufReader::new(f);
        let mut line_number = 0;

        for line in f.lines() {
            let line_string = line.unwrap();
            line_number = line_number + 1;

            let captures = regexp.captures(&line_string).unwrap();
            let key = captures.get(1).unwrap().as_str();
            let value = captures.get(2).unwrap().as_str();

            println!("{}:{} ({}:{})", file.path().file_name().unwrap().to_str().unwrap(), line_number, key, value);
        }
    }
}
