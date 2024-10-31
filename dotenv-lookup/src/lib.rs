use crate::file::Files;
use std::path::PathBuf;

mod file;
mod fs_utils;
mod line;
mod quote_type;

pub use crate::file::FileEntry;
pub use crate::line::LineEntry;

pub struct Dotenv<'a> {
    dir: &'a PathBuf,
    paths: Vec<PathBuf>,
    recursive: bool,
    excluded: Vec<PathBuf>,
}

pub fn new<'a>(dir: &'a PathBuf, paths: &'a [&'a PathBuf]) -> Dotenv<'a> {
    let mut paths = paths
        .iter()
        .filter_map(|f| fs_utils::canonicalize(f).ok())
        .collect::<Vec<_>>();

    if paths.is_empty() {
        paths.push(dir.to_path_buf());
    }

    Dotenv {
        dir,
        paths,
        recursive: false,
        excluded: vec![],
    }
}

impl<'a> Dotenv<'a> {
    pub fn exclude(self, exclude: &'a [&'a PathBuf]) -> Self {
        let excluded = exclude
            .iter()
            .filter_map(|f| fs_utils::canonicalize(f).ok())
            .collect::<Vec<_>>();

        Self { excluded, ..self }
    }

    pub fn recursive(self, recursive: bool) -> Self {
        Self { recursive, ..self }
    }

    pub fn lookup_files(self) -> Files {
        let files = lookup_dotenv_paths(self.paths, self.excluded.as_slice(), self.recursive)
            .iter()
            .filter_map(|path: &PathBuf| -> Option<(FileEntry, Vec<LineEntry>)> {
                fs_utils::get_relative_path(path, self.dir).and_then(FileEntry::from)
            })
            .collect();

        Files::new(files)
    }
}

fn lookup_dotenv_paths(
    dir_entries: Vec<PathBuf>,
    excludes: &[PathBuf],
    is_recursive: bool,
) -> Vec<PathBuf> {
    let nested_paths: Vec<PathBuf> = dir_entries
        .iter()
        .filter(|entry| entry.is_dir())
        .filter(|entry| !excludes.contains(entry))
        .filter_map(|dir| dir.read_dir().ok())
        .map(|read_dir| {
            read_dir
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|path| {
                    file::is_dotenv_file(path)
                        || (is_recursive && path.is_dir() && path.read_link().is_err())
                })
                .collect()
        })
        .flat_map(|dir_entries| lookup_dotenv_paths(dir_entries, excludes, is_recursive))
        .collect();

    let mut file_paths: Vec<PathBuf> = dir_entries
        .into_iter()
        .filter(|entry| entry.is_file())
        .filter(|entry| !excludes.contains(entry))
        .collect();

    file_paths.extend(nested_paths);
    file_paths.sort();
    file_paths.dedup();
    file_paths
}

pub fn from_stdin() -> Option<(FileEntry, Vec<LineEntry>)> {
    FileEntry::from_stdin()
}

pub fn is_escaped(prefix: &str) -> bool {
    prefix.chars().rev().take_while(|ch| *ch == '\\').count() % 2 == 1
}
