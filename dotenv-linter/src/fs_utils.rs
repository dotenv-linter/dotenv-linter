use std::{
    fs::{copy, File},
    io::{self, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};

use dotenv_core::LineEntry;
use dotenv_finder::FileEntry;

use crate::Result;

/// In the future versions we should create a backup copy, or at least notify the user about it
pub fn write_file(path: &Path, lines: Vec<LineEntry>) -> io::Result<()> {
    let mut file = File::create(path)?;

    // We don't write the last line, because it contains only LF (common::FileEntry::from)
    // and writeln! already adds LF.
    for line in lines[..lines.len() - 1].iter() {
        writeln!(file, "{}", line.raw_string)?;
    }

    Ok(())
}

pub fn backup_file(fe: &FileEntry) -> Result<PathBuf> {
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

// #[cfg(test)]
// mod tests {
//     use std::fs;
//
//     use super::*;
//     use crate::common::tests::*;
//
//     #[test]
//     fn write_file_test() {
//         let file_name = ".env";
//         let dir = tempfile::tempdir().expect("create temp dir");
//         let path = dir.path().join(file_name);
//
//         let lines = vec![
//             line_entry(1, 3, "A=B"),
//             line_entry(2, 3, "Z=Y"),
//             blank_line_entry(3, 3),
//         ];
//
//         assert!(write_file(&path, lines).is_ok());
//         assert_eq!(
//             b"A=B\nZ=Y\n",
//             fs::read(path.as_path()).expect("file read").as_slice()
//         );
//
//         dir.close().expect("temp dir deleted");
//     }
//
//     #[test]
//     fn backup_file_test() {
//         let file_name = String::from(".env");
//         let dir = tempfile::tempdir().expect("create temp dir");
//         let path = dir.path().join(&file_name);
//
//         let fe = FileEntry {
//             path,
//             file_name,
//             total_lines: 3,
//         };
//
//         let lines = vec![
//             line_entry(1, 3, "A=B"),
//             line_entry(2, 3, "Z=Y"),
//             blank_line_entry(3, 3),
//         ];
//
//         if write_file(&fe.path, lines).is_ok() {
//             match backup_file(&fe) {
//                 Ok(path) => {
//                     assert_eq!(
//                         b"A=B\nZ=Y\n",
//                         fs::read(path.as_path()).expect("file read").as_slice()
//                     );
//                     assert_ne!(path, fe.path);
//                 }
//                 Err(_) => panic!("could not copy file - test failed"),
//             }
//         } else {
//             panic!("could not write file - test failed")
//         }
//
//         dir.close().expect("temp dir deleted");
//     }
// }
