use crate::common::FileEntry;
use std::ffi::OsString;

/// Prefix for the backup output.
const BACKUP_PREFIX: &str = "Original file was backed up to: ";

pub struct FixOutput {
    // Quiet program output mode
    is_quiet_mode: bool,
}

impl FixOutput {
    pub fn new(is_quiet_mode: bool) -> Self {
        FixOutput { is_quiet_mode }
    }

    /// Prints information about a file in process
    pub fn print_processing_info(&self, file: &FileEntry) {
        if !&self.is_quiet_mode {
            println!("Fixing {}", file);
        }
    }

    pub fn print_total(&self, total: usize) {
        if total != 0 {
            println!("\nAll warnings are fixed. Total: {}", total);
        } else {
            println!("\nNo warnings found");
        }
    }

    /// Prints the backup file's path.
    pub fn print_backup(&self, backup_path: &OsString) {
        println!("{}{:?}", BACKUP_PREFIX, backup_path);
        if !self.is_quiet_mode {
            println!();
        }
    }
}
