use std::fmt;
use std::fs;
use std::path::PathBuf;

use crate::common::*;

const EXCLUDED_FILES: &[&str] = &[".envrc"];

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
    pub fn from(path: PathBuf) -> Option<(Self, Vec<String>)> {
        let file_name = Self::get_file_name(&path)?.to_string();

        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => return None,
        };

        let mut lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();

        // You must add a line, because [`Lines`] does not return the last empty row (excludes LF)
        if content.ends_with(LF) {
            lines.push(LF.to_string());
        }

        Some((
            FileEntry {
                path,
                file_name,
                total_lines: lines.len(),
            },
            lines,
        ))
    }

    /// Checks a file name with the `.env` pattern
    pub fn is_env_file(path: &PathBuf) -> bool {
        let pattern = ".env";
        Self::get_file_name(path)
            .filter(|file_name| !EXCLUDED_FILES.contains(file_name))
            .filter(|file_name| file_name.starts_with(pattern) || file_name.ends_with(pattern))
            .is_some()
    }

    fn get_file_name<'a>(path: &'a PathBuf) -> Option<&'a str> {
        path.file_name().and_then(|file_name| file_name.to_str())
    }
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
        ];

        assertions.extend(EXCLUDED_FILES.iter().map(|file| (*file, false)));

        for (file_name, expected) in assertions {
            assert_eq!(
                expected,
                FileEntry::is_env_file(&PathBuf::from(file_name)),
                "Expected {} for the file name {}",
                expected,
                file_name
            )
        }
    }
}
