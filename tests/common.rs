use std::path::Path;

pub fn relative_path<'a>(current_dir: &'a Path, path: &'a Path) -> &'a Path {
    path.strip_prefix(&current_dir).unwrap()
}
