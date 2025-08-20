/// For other platforms - continue use `std:fs`
#[cfg(not(windows))]
pub use std::fs::canonicalize;
use std::path::{Path, PathBuf};

/// For the Windows platform, we need to remove the UNC prefix.
#[cfg(windows)]
pub use dunce::canonicalize;

/// Returns the relative path for `target_path` relative to `base_path`
pub(crate) fn get_relative_path(target_path: &Path, base_path: &Path) -> Option<PathBuf> {
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

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    fn run_relative_path_asserts(assertions: Vec<(&str, &str, &str)>) {
        for (target, base, relative) in assertions {
            assert_eq!(
                get_relative_path(&PathBuf::from(target), &PathBuf::from(base)),
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
}
