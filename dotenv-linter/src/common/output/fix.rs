use crate::common::Warning;
use colored::*;
use dotenv_lookup::FileEntry;
use std::path::Path;

/// Prefix for the backup output
const BACKUP_PREFIX: &str = "Original file was backed up to: ";

pub struct FixOutput {
    // Quiet program output mode
    is_quiet_mode: bool,
    // Total number of files to check
    files_count: usize,
}

impl FixOutput {
    pub fn new(is_quiet_mode: bool) -> Self {
        FixOutput {
            is_quiet_mode,
            files_count: 0,
        }
    }

    pub(crate) fn files_count(self, files_count: usize) -> Self {
        Self {
            files_count,
            ..self
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
    pub fn print_backup(&self, backup_path: &Path) {
        println!("{}{:?}", BACKUP_PREFIX, backup_path);
        if !self.is_quiet_mode {
            println!();
        }
    }

    /// Prints warnings without any additional information
    pub fn print_warnings(&self, file: &FileEntry, warnings: &[Warning], file_index: usize) {
        if self.is_quiet_mode {
            return;
        }

        warnings
            .iter()
            .for_each(|w| println!("{}{}", format!("{}:", file).italic(), w));

        let is_last_file = file_index == self.files_count - 1;
        if !warnings.is_empty() && !is_last_file {
            println!();
        }
    }

    /// Prints no files found message
    pub fn print_nothing_to_fix(&self) {
        if self.is_quiet_mode || self.files_count > 0 {
            return;
        }

        println!("Nothing to fix");
    }

    /// Prints not all warnings fixed message
    pub fn print_not_all_warnings_fixed(&self) {
        if self.is_quiet_mode {
            return;
        }

        println!("{}", "Could not fix all warnings".red().bold());
    }
}
