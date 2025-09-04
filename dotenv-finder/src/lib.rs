use std::path::PathBuf;

use dotenv_core::LineEntry;

use crate::file::Files;

mod file;
mod fs;
mod quote;

pub use crate::file::FileEntry;

pub struct Finder<'a> {
    dir: &'a PathBuf,
    paths: Vec<PathBuf>,
    excluded: Vec<PathBuf>,
    recursive: bool,
}

pub struct FinderBuilder<'a> {
    dir: &'a PathBuf,
    paths: Vec<PathBuf>,
    excluded: Vec<PathBuf>,
    recursive: bool,
}

impl<'a> FinderBuilder<'a> {
    pub fn new(dir: &'a PathBuf) -> Self {
        Self {
            dir,
            paths: vec![],
            excluded: vec![],
            recursive: false,
        }
    }

    pub fn with_paths(mut self, paths: &'a [&'a PathBuf]) -> Self {
        self.paths = paths
            .iter()
            .filter_map(|f| fs::canonicalize(f).ok())
            .collect();

        if self.paths.is_empty() {
            self.paths.push(self.dir.clone());
        }

        self
    }

    pub fn exclude(mut self, exclude: &'a [&'a PathBuf]) -> Self {
        self.excluded = exclude
            .iter()
            .filter_map(|f| fs::canonicalize(f).ok())
            .collect();
        self
    }

    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    pub fn build(self) -> Finder<'a> {
        Finder {
            dir: self.dir,
            paths: self.paths,
            excluded: self.excluded,
            recursive: self.recursive,
        }
    }
}

impl<'a> Finder<'a> {
    pub fn find(&self) -> Files {
        let files = find_dotenv_paths(self.paths.clone(), self.excluded.as_slice(), self.recursive)
            .iter()
            .filter_map(|path: &PathBuf| -> Option<(FileEntry, Vec<LineEntry>)> {
                fs::get_relative_path(path, self.dir).and_then(FileEntry::from)
            })
            .collect();

        Files::new(files)
    }
}

fn find_dotenv_paths(
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
        .flat_map(|dir_entries| find_dotenv_paths(dir_entries, excludes, is_recursive))
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
