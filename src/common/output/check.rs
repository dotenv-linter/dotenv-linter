use colored::*;

use crate::common::{FileEntry, Warning};

pub struct CheckOutput {
    // Quiet program output mode
    is_quiet_mode: bool,
    // Total number of files to check
    files_count: usize,
}

impl CheckOutput {
    pub fn new(is_quiet_mode: bool, files_count: usize) -> Self {
        CheckOutput {
            is_quiet_mode,
            files_count,
        }
    }

    /// Prints a message that there is nothing to check
    pub fn print_nothing_to_check(&self) {
        if !self.is_quiet_mode {
            println!("Nothing to check");
        }
    }

    /// Prints information about a file in process
    pub fn print_processing_info(&self, file: &FileEntry) {
        if !self.is_quiet_mode {
            println!("Checking {}", file);
        }
    }

    /// Prints warnings without any additional information
    pub fn print_warnings(&self, warnings: &[Warning], file_index: usize) {
        warnings.iter().for_each(|w| println!("{}", w));

        if self.is_quiet_mode {
            return;
        }

        let is_last_file = file_index == self.files_count - 1;
        if !warnings.is_empty() && !is_last_file {
            println!();
        }
    }

    pub fn print_total(&self, total: usize) {
        if self.is_quiet_mode {
            return;
        }

        if total != 0 {
            let mut problems = String::from("problem");

            if total != 1 {
                problems += "s";
            }

            println!(
                "\n{}",
                format!("{} {} {}", "Found", total, problems).red().bold()
            );
        } else {
            println!("\n{}", "No problems found".green().bold());
        }
    }
}
