use std::{
    io::{self, Read},
    path::Path,
};

use crate::Result;

pub(crate) const STDIN_PATH: &str = "-";

pub(crate) fn is_stdin(path: &Path) -> bool {
    path == Path::new(STDIN_PATH)
}

pub(crate) fn read() -> Result<String> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;
    Ok(content)
}
