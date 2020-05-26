use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::common::LineEntry;

// I think that we should create a backup copy
pub fn write(path: &PathBuf, lines: Vec<LineEntry>) -> io::Result<()> {
    let mut file = File::create(path)?;

    for line in lines[..lines.len() - 1].iter() {
        writeln!(file, "{}", line.raw_string)?;
    }

    Ok(())
}
