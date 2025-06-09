use clap::ArgMatches;
use std::fs;
use std::io::{self, Read};

/// Run either stdin-mode or file-mode
pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    if matches.get_flag("stdin") {
        let filename = matches
            .get_one::<String>("stdin-filename")
            .map(String::as_str)
            .unwrap_or(".env");
        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;
        lint_content(&content, filename)?;
    } else {
        let paths: Vec<String> = match matches.get_many::<String>("path") {
            Some(vals) => vals.map(Clone::clone).collect(),
            None => vec![".env".into()],
        };
        for path in paths {
            let content = fs::read_to_string(&path)?;
            lint_content(&content, &path)?;
        }
    }
    Ok(())
}

fn lint_content(content: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // … your existing lint logic …
    println!("Linting {} ({} bytes)", filename, content.len());
    Ok(())
}
