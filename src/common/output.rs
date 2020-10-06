use crate::common::{FileEntry, Mode, Warning};
use std::ffi::OsString;
use std::fmt;

/// Wraps warnings to provide more information when printing.
pub struct Output {
    /// Path of the file the warnings originated from.
    path: FileEntry,

    /// Path of the file's backup.
    backup_path: Option<OsString>,

    /// List of warnings.
    pub warnings: Vec<Warning>,

    /// Mode of the program.
    mode: Mode,
}

impl Output {
    pub fn new(
        path: FileEntry,
        backup_path: Option<OsString>,
        warnings: Vec<Warning>,
        mode: Mode,
    ) -> Self {
        Self {
            path,
            backup_path,
            warnings,
            mode,
        }
    }

    /// Prints warnings without any additional information.
    pub fn print_warnings(&self) {
        self.warnings.iter().for_each(|w| println!("{}", w));
    }

    /// Prints the backup file's path.
    pub fn print_backup(&self) {
        if let Some(p) = &self.backup_path {
            println!("Original file was backed up to: {:?}", p);
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mode {
            Mode::Fix => {
                write!(f, "Fixing {}", self.path)?;
            }
            Mode::Check => {
                write!(f, "Checking {}", self.path)?;
            }
        }
        if self.backup_path != None {
            writeln!(f)?;
        }
        self.print_backup();
        if !self.warnings.is_empty() {
            writeln!(f)?;
        }
        for w in self.warnings.iter() {
            writeln!(f, "{}", w)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn output_fmt_test() {
        let line = line_entry(1, 1, "FOO=BAR");
        let warning = Warning::new(
            line.clone(),
            "DuplicatedKey",
            String::from("The FOO key is duplicated"),
        );
        let output = Output::new(line.file, None, vec![warning], Mode::Check);

        assert_eq!(
            "Checking .env\n.env:1 DuplicatedKey: The FOO key is duplicated\n",
            format!("{}", output)
        );
    }
}
