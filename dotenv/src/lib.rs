use crate::file_entry::FileEntry;
use crate::line_entry::LineEntry;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

mod file_entry;
mod line_entry;
mod quote_type;

pub const LF: &str = "\n";

fn is_escaped(prefix: &str) -> bool {
    prefix.chars().rev().take_while(|ch| *ch == '\\').count() % 2 == 1
}

pub(crate) struct DotenvOptions<'a> {
    input: Vec<PathBuf>,
    current_dir: &'a Path,
    is_recursive: bool,
    excluded: Option<Vec<PathBuf>>,
}

pub(crate) struct DotenvFiles {
    files: BTreeMap<FileEntry, Vec<LineEntry>>,
}

impl DotenvFiles {
    pub(crate) fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub(crate) fn count(&self) -> usize {
        self.files.len()
    }
}

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

    pub(crate) fn lookup_files(self) -> DotenvFiles {
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

        DotenvFiles { files }
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
                    file_entry::is_dotenv_file(path)
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
