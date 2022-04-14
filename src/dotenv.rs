use std::path::{Path, PathBuf};

pub(crate) struct DotenvOptions<'a> {
    input: Vec<PathBuf>,
    current_dir: &'a Path,
    is_recursive: bool,
    excluded: Option<Vec<PathBuf>>,
}

pub(crate) struct DotenvFiles {}

pub(crate) fn new(input: Vec<PathBuf>, current_dir: &Path) -> DotenvOptions {
    DotenvOptions {
        input,
        current_dir,
        excluded: None,
        is_recursive: false,
    }
}

impl<'a> DotenvOptions<'a> {
    pub(crate) fn recursive(self, is_recursive: bool) -> Self {
        Self {
            is_recursive,
            ..self
        }
    }

    pub(crate) fn exclude(self, excluded: Vec<PathBuf>) -> Self {
        Self {
            excluded: Some(excluded),
            ..self
        }
    }

    pub(crate) fn parse(self) -> DotenvFiles {
        DotenvFiles {}
    }
}
