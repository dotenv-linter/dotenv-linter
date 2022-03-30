struct GetLinesOptions<'a> {
    input: Vec<PathBuf>,
    current_dir: &'a Path,
    excluded: Option<Vec<PathBuf>>,
    is_recursive: bool,
}

impl<'a> GetLinesOptions<'a> {
    fn new(input: Vec<PathBuf>, current_dir: &'a Path) -> Self {
        Self {
            input,
            current_dir,
            excluded: None,
            is_recursive: false,
        }
    }

    fn excluded(self, excluded: Vec<PathBuf>) -> Self {
        Self {
            excluded: Some(excluded),
            ..self
        }
    }

    fn recursive(self, is_recursive: bool) -> Self {
        Self {
            is_recursive,
            ..self
        }
    }
}
