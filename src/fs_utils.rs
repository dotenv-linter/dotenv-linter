use std::path::PathBuf;

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

    let mut i = 0;
    for (b, t) in comp_base.iter().zip(comp_target.iter()) {
        if b != t {
            break;
        }
        i += 1;
    }

    let mut relative_path = PathBuf::new();

    for _ in 0..(comp_base.len() - i) {
        relative_path.push("..");
    }
    relative_path.extend(comp_target.get(i..)?);

    Some(relative_path)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
