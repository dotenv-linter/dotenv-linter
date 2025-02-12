use crate::is_escaped;
use crate::line::LineEntry;
use crate::quote_type::is_multiline_start;
use std::collections::btree_map::IntoIter;
use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};
use std::{fmt, fs};

const PATTERN: &str = ".env";
const EXCLUDED_FILES: &[&str] = &[".envrc"];
const BACKUP_EXTENSION: &str = ".bak";
pub const LF: &str = "\n";

pub struct Files(BTreeMap<FileEntry, Vec<LineEntry>>);

impl Files {
    pub fn new(files: BTreeMap<FileEntry, Vec<LineEntry>>) -> Self {
        Self(files)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl IntoIterator for Files {
    type Item = (FileEntry, Vec<LineEntry>);
    type IntoIter = IntoIter<FileEntry, Vec<LineEntry>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FileEntry {
    pub path: PathBuf,
    pub file_name: String,
    pub total_lines: usize,
}

impl fmt::Display for FileEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

impl FileEntry {
    /// Converts `PathBuf` to tuple of `(FileEntry, Vec<LineEntry>)`
    pub fn from(path: PathBuf) -> Option<(Self, Vec<LineEntry>)> {
        let file_name = get_file_name(&path)?.to_string();
        let content = fs::read_to_string(&path).ok()?;
        let lines = get_line_entries(content);

        Some((
            FileEntry {
                path,
                file_name,
                total_lines: lines.len(),
            },
            lines,
        ))
    }

    pub fn from_stdin() -> Option<(Self, Vec<LineEntry>)> {
        let path = PathBuf::from("");
        let content = io::read_to_string(io::stdin()).ok()?;
        let lines = get_line_entries(content);

        Some((
            FileEntry {
                path,
                file_name: String::new(),
                total_lines: lines.len(),
            },
            lines,
        ))
    }
}

/// Checks a file name with the `.env` pattern
pub(crate) fn is_dotenv_file(path: &Path) -> bool {
    get_file_name(path)
        .filter(|file_name| !EXCLUDED_FILES.contains(file_name))
        .filter(|file_name| file_name.starts_with(PATTERN) || file_name.ends_with(PATTERN))
        .filter(|file_name| !file_name.ends_with(BACKUP_EXTENSION))
        .is_some()
}

fn get_file_name(path: &Path) -> Option<&str> {
    path.file_name().and_then(|file_name| file_name.to_str())
}

fn get_line_entries(content: String) -> Vec<LineEntry> {
    let mut lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();

    // You must add a line, because [`Lines`] does not return the last empty row (excludes LF)
    if content.ends_with(LF) {
        lines.push(LF.to_string());
    }

    let length = lines.len();

    let mut lines: Vec<LineEntry> = lines
        .into_iter()
        .enumerate()
        .map(|(index, line)| LineEntry::new(index + 1, line, length == (index + 1)))
        .collect();

    reduce_multiline_entries(&mut lines);
    lines
}

fn reduce_multiline_entries(lines: &mut Vec<LineEntry>) {
    let length = lines.len();
    let multiline_ranges = find_multiline_ranges(lines);

    // Replace multiline value to one line-entry for checking
    let mut offset = 1; // index offset to account deleted lines (for access by index)
    for (start, end) in multiline_ranges {
        let result = lines
            .drain(start - offset..end - offset + 1) // TODO: consider `drain_filter` (after stabilization in rust std)
            .map(|entry| entry.raw_string)
            .reduce(|result, line| result + "\n" + &line); // TODO: `intersperse` (after stabilization in rust std)

        if let Some(value) = result {
            lines.insert(start - offset, LineEntry::new(start, value, length == end));
        }

        offset += end - start;
    }
}

fn find_multiline_ranges(lines: &[LineEntry]) -> Vec<(usize, usize)> {
    let mut multiline_ranges: Vec<(usize, usize)> = Vec::new();
    let mut start_number: Option<usize> = None;
    let mut quote_char: Option<char> = None;

    // here we find ranges of multi-line values
    lines.iter().for_each(|entry| {
        if let Some(start) = start_number {
            if let Some(quote_char) = quote_char {
                if let Some(idx) = entry.raw_string.find(quote_char) {
                    if !is_escaped(&entry.raw_string[..idx]) {
                        multiline_ranges.push((start, entry.number));
                        start_number = None;
                    }
                }
            }
        } else if let Some(trimmed_value) = entry.get_value().map(|val| val.trim()) {
            if let Some(quote_type) = is_multiline_start(trimmed_value) {
                quote_char = Some(quote_type.char());
                start_number = Some(entry.number);
            }
        }
    });

    multiline_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from {
        use super::*;

        #[test]
        fn path_without_file_test() {
            let f = FileEntry::from(PathBuf::from("/"));
            assert_eq!(None, f);
        }

        #[test]
        fn path_with_file_test() {
            let file_name = String::from(".env");
            let dir = tempfile::tempdir().expect("create temp dir");
            let path = dir.path().join(&file_name);
            fs::File::create(&path).expect("create testfile");

            let f = FileEntry::from(path.clone());
            assert_eq!(
                Some((
                    FileEntry {
                        path,
                        file_name,
                        total_lines: 0
                    },
                    vec![]
                )),
                f
            );
            dir.close().expect("temp dir deleted");
        }
    }

    #[test]
    fn is_env_file_test() {
        let mut assertions = vec![
            (".env", true),
            ("foo.env", true),
            (".env.foo", true),
            (".env.foo.common", true),
            ("env", false),
            ("env.foo", false),
            ("foo_env", false),
            ("foo-env", false),
            (".my-env-file", false),
            ("dev.env.js", false),
            (".env.bak", false),
        ];

        assertions.extend(EXCLUDED_FILES.iter().map(|file| (*file, false)));

        for (file_name, expected) in assertions {
            assert_eq!(
                expected,
                is_dotenv_file(&PathBuf::from(file_name)),
                "Expected {} for the file name {}",
                expected,
                file_name
            )
        }
    }
}
