use crate::common::{FileEntry, LineEntry};
use std::error::Error;
use std::fs::{copy, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::SystemTime;

/// For the Windows platform, we need to remove the UNC prefix.
#[cfg(windows)]
pub use dunce::canonicalize;

/// For other platforms - continue use `std:fs`
#[cfg(not(windows))]
pub use std::fs::canonicalize;

/// Returns the relative path for `target_path` relative to `base_path`
pub fn get_relative_path(target_path: &PathBuf, base_path: &PathBuf) -> Option<PathBuf> {
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

/// In the future versions we should create a backup copy, or at least notify the user about it
pub fn write_file(path: &PathBuf, lines: Vec<LineEntry>) -> io::Result<()> {
    let mut file = File::create(path)?;

    // We don't write the last line, because it contains only LF (common::FileEntry::from)
    // and writeln! already adds LF.
    for line in lines[..lines.len() - 1].iter() {
        writeln!(file, "{}", line.raw_string)?;
    }

    Ok(())
}

pub fn backup_file(fe: &FileEntry) -> Result<PathBuf, Box<dyn Error>> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let mut new_path = fe.path.to_owned();
    new_path.set_file_name(format!("{}_{}.bak", &fe.file_name, timestamp));

    copy(&fe.path, &new_path)
        .map(|_| new_path)
        .map_err(Box::new)
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;
    use std::fs::{self, File};

    fn run_relative_path_asserts(assertions: Vec<(&str, &str, &str)>) {
        for (target, base, relative) in assertions {
            assert_eq!(
                get_relative_path(&PathBuf::from(target), &PathBuf::from(base),),
                Some(PathBuf::from(relative))
            );
        }
    }

    #[test]
    #[cfg(not(windows))]
    fn test_relative_path() {
        let assertions = vec![
            ("/a/.env", "/a", ".env"),
            ("/a/b/.env", "/a", "b/.env"),
            ("/.env", "/a/b/c", "../../../.env"),
            ("/a/b/c/d/.env", "/a/b/e/f", "../../c/d/.env"),
        ];

        run_relative_path_asserts(assertions)
    }

    #[test]
    #[cfg(windows)]
    fn test_relative_path() {
        let assertions = vec![
            ("C:\\a\\.env", "C:\\a", ".env"),
            ("\\a\\b\\.env", "\\a", "b\\.env"),
            ("\\.env", "\\a\\b\\c", "..\\..\\..\\.env"),
            ("C:\\a\\b\\c\\.env", "C:\\a\\b\\e\\f", "..\\..\\c\\.env"),
            ("\\\\?\\C:\\a\\.env", "C:\\a\\b", "\\\\?\\C:\\a\\.env"),
        ];

        run_relative_path_asserts(assertions)
    }

    #[test]
    #[cfg(not(windows))]
    fn test_canonicalize() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join(".env");
        File::create(&path).expect("file created");

        let fs_canonical_path = std::fs::canonicalize(&path).expect("canonical path by std::fs");
        let canonical_path = super::canonicalize(&path).expect("canonical path by fs_utils");

        // The result of `fs_utils` must match `std::fs` for non-Windows platform
        assert_eq!(canonical_path, fs_canonical_path);

        dir.close().expect("temp dir deleted");
    }

    #[test]
    #[cfg(windows)]
    fn test_canonicalize() {
        const UNC_PREFIX: &str = "\\\\?\\";

        let file_name = ".env";
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join(file_name);
        File::create(&path).expect("create testfile");

        let dunce_canonical_path = dunce::canonicalize(&path).expect("canonical path by `dunce`");
        let canonical_path = super::canonicalize(&path).expect("canonical path by `fs_utils`");

        let contains_unc = canonical_path
            .to_str()
            .filter(|path| path.contains(UNC_PREFIX))
            .is_some();

        // The result of `fs_utils` must match `dunce` on Windows
        assert_eq!(canonical_path, dunce_canonical_path);
        assert!(!contains_unc);

        dir.close().expect("temp dir deleted");
    }

    #[test]
    fn write_file_test() {
        let file_name = ".env";
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join(file_name);

        let lines = vec![
            line_entry(1, 3, "A=B"),
            line_entry(2, 3, "Z=Y"),
            blank_line_entry(3, 3),
        ];

        assert!(write_file(&path, lines).is_ok());
        assert_eq!(
            b"A=B\nZ=Y\n",
            fs::read(path.as_path()).expect("file read").as_slice()
        );

        dir.close().expect("temp dir deleted");
    }

    #[test]
    fn backup_file_test() {
        let file_name = String::from(".env");
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join(&file_name);

        let fe = FileEntry {
            path,
            file_name,
            total_lines: 3,
        };

        let lines = vec![
            line_entry(1, 3, "A=B"),
            line_entry(2, 3, "Z=Y"),
            blank_line_entry(3, 3),
        ];

        if write_file(&fe.path, lines).is_ok() {
            match backup_file(&fe) {
                Ok(path) => {
                    assert_eq!(
                        b"A=B\nZ=Y\n",
                        fs::read(path.as_path()).expect("file read").as_slice()
                    );
                    assert_ne!(path, fe.path);
                }
                Err(_) => panic!("could not copy file - test failed"),
            }
        } else {
            panic!("could not write file - test failed")
        }

        dir.close().expect("temp dir deleted");
    }
}
