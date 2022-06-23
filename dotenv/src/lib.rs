use std::collections::btree_map::IntoIter;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

mod file;
mod line;
mod quote_type;

pub use crate::file::FileEntry;
pub use crate::line::LineEntry;

pub(crate) const LF: &str = "\n";

fn is_escaped(prefix: &str) -> bool {
    prefix.chars().rev().take_while(|ch| *ch == '\\').count() % 2 == 1
}

pub struct Options<'a> {
    input: Vec<PathBuf>,
    current_dir: &'a Path,
    is_recursive: bool,
    excluded: Option<Vec<PathBuf>>,
}

pub struct Files {
    files: BTreeMap<FileEntry, Vec<LineEntry>>,
}

impl Files {
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub fn count(&self) -> usize {
        self.files.len()
    }
}

impl IntoIterator for Files {
    type Item = (FileEntry, Vec<LineEntry>);
    type IntoIter = IntoIter<FileEntry, Vec<LineEntry>>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}

pub fn new(input: Vec<PathBuf>, current_dir: &Path) -> Options {
    Options {
        input,
        current_dir,
        excluded: None,
        is_recursive: false,
    }
}

impl<'a> Options<'a> {
    pub fn recursive(mut self, is_recursive: bool) -> Self {
        self.is_recursive = is_recursive;
        self

        // Self {
        //     is_recursive,
        //     ..self
        // }
    }

    pub fn exclude(self, excluded: Vec<PathBuf>) -> Self {
        // todo
        Self {
            excluded: Some(excluded),
            ..self
        }
    }

    pub fn lookup_files(self) -> Files {
        let files = lookup_dotenv_paths(
            self.input,
            &self.excluded.unwrap_or_default(),
            self.is_recursive,
        )
        .iter()
        .filter_map(|path: &PathBuf| -> Option<(FileEntry, Vec<LineEntry>)> {
            get_relative_path(path, self.current_dir).and_then(FileEntry::from)
        })
        .collect();

        Files { files }
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

/// Returns the relative path for `target_path` relative to `base_path`
fn get_relative_path(target_path: &Path, base_path: &Path) -> Option<PathBuf> {
    let comp_target: Vec<_> = target_path.components().collect();
    let comp_base: Vec<_> = base_path.components().collect();

    let i = comp_base
        .iter()
        .zip(comp_target.iter())
        .take_while(|(b, t)| b == t)
        .count();

    let mut relative_path = (0..(comp_base.len() - i)).fold(PathBuf::new(), |mut acc, _| {
        acc.push("..");
        acc
    });
    relative_path.extend(comp_target.get(i..)?);

    Some(relative_path)
}
