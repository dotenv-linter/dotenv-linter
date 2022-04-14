use crate::quote_type::QuoteType;
use crate::{is_escaped, LF};
use std::path::{Path, PathBuf};
use std::{fmt, fs};

const PATTERN: &str = ".env";
const EXCLUDED_FILES: &[&str] = &[".envrc"];
const BACKUP_EXTENSION: &str = ".bak";

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
    /// Converts `PathBuf` to tuple of `(FileEntry, Vec<String>)`
    pub fn from(path: PathBuf) -> Option<(Self, Vec<LineEntry>)> {
        let file_name = get_file_name(&path)?.to_string();
        let content = fs::read_to_string(&path).ok()?;

        let mut lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();

        // You must add a line, because [`Lines`] does not return the last empty row (excludes LF)
        if content.ends_with(LF) {
            lines.push(LF.to_string());
        }

        let lines = get_line_entries(lines);

        Some((
            FileEntry {
                path,
                file_name,
                total_lines: lines.len(),
            },
            lines,
        ))
    }
}

fn get_line_entries(lines: Vec<String>) -> Vec<LineEntry> {
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

/// Returns the `QuoteType` for a `&str` starting with a quote-char
fn is_multiline_start(val: &str) -> Option<QuoteType> {
    for quote_type in [QuoteType::Single, QuoteType::Double] {
        if quote_type.is_quoted_value(val) {
            return Some(quote_type);
        }
    }
    None
}
