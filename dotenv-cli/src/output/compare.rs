use colored::Colorize;
use dotenv_finder::FileEntry;

use crate::compare::CompareWarning;

pub struct CompareOutput {
    // Quiet program output mode
    is_quiet_mode: bool,
}

impl CompareOutput {
    pub fn new(is_quiet_mode: bool) -> Self {
        CompareOutput { is_quiet_mode }
    }

    /// Prints information about a file in process
    pub fn print_processing_info(&self, file: &FileEntry) {
        if !self.is_quiet_mode {
            println!("Comparing {file}");
        }
    }

    /// Prints warnings without any additional information
    pub fn print_warnings(&self, warnings: &[CompareWarning]) {
        warnings.iter().for_each(|w| {
            println!(
                "{} is missing keys: {}",
                w.path().display().to_string().italic(),
                w.missing_keys()
                    .iter()
                    .map(|k| k.red().bold().to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        })
    }

    /// Prints 'Nothing to compare' in the absence of '.env' files for compare
    pub fn print_nothing_to_compare(&self) {
        if !self.is_quiet_mode {
            println!("Nothing to compare");
        }
    }

    /// Prints "No difference found" when the two compared files have no difference
    pub fn print_no_difference_found(&self) {
        if !self.is_quiet_mode {
            println!("No difference found");
        }
    }
}
