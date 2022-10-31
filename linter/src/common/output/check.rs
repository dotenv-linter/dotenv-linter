use crate::common::Warning;
use colored::*;
use dotenv::FileEntry;

pub(crate) struct CheckOutput {
    // Quiet program output mode
    is_quiet_mode: bool,
    // Total number of files to check
    files_count: usize,
}

impl CheckOutput {
    pub(crate) fn new(is_quiet_mode: bool) -> Self {
        Self {
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

    /// Prints a message that there is nothing to check
    pub(crate) fn print_nothing_to_check(&self) {
        if !self.is_quiet_mode {
            println!("Nothing to check");
        }
    }

    /// Prints information about a file in process
    pub(crate) fn print_processing_info(&self, file: &FileEntry) {
        if !self.is_quiet_mode {
            println!("Checking {}", file);
        }
    }

    /// Prints warnings without any additional information
    pub(crate) fn print_warnings(&self, file: &FileEntry, warnings: &[Warning], file_index: usize) {
        warnings
            .iter()
            .for_each(|w| println!("{}{}", format!("{}:", file).italic(), w));

        if self.is_quiet_mode {
            return;
        }

        let is_last_file = file_index == self.files_count - 1;
        if !warnings.is_empty() && !is_last_file {
            println!();
        }
    }

    pub(crate) fn print_total(&self, total: usize) {
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
