use crate::common::{FileEntry, Warning};
use std::ffi::OsString;

/// Prefix for the backup output
const BACKUP_PREFIX: &str = "Original file was backed up to: ";

pub struct FixOutput {
    // Quiet program output mode
    is_quiet_mode: bool,
    // Total number of files to check
    files_count: usize,
}

impl FixOutput {
    pub fn new(is_quiet_mode: bool, files_count: usize) -> Self {
        FixOutput {
            is_quiet_mode,
            files_count,
        }
    }

    /// Prints information about a file in process
    pub fn print_processing_info(&self, file: &FileEntry) {
        if !self.is_quiet_mode {
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

    /// Prints the backup file's path
    pub fn print_backup(&self, backup_path: &OsString) {
        println!("{}{:?}", BACKUP_PREFIX, backup_path);
        if !self.is_quiet_mode {
            println!();
        }
    }

    /// Prints warnings without any additional information
    pub fn print_warnings(&self, warnings: &[Warning], file_index: usize) {
        if self.is_quiet_mode {
            return;
        }

        warnings.iter().for_each(|w| println!("{}", w));
        let is_last_file = file_index == self.files_count - 1;
        if !warnings.is_empty() && !is_last_file {
            println!();
        }
    }
}
