use crate::common::{CompareWarning, FileEntry};

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
            println!("Comparing {}", file);
        }
    }

    /// Prints warnings without any additional information
    pub fn print_warnings(&self, warnings: &[CompareWarning]) {
        warnings.iter().for_each(|w| println!("{}", w))
    }

    /// Prints 'Nothing to compare' in the absence of '.env' files for compare
    pub fn print_nothing_to_compare(&self) {
        if !self.is_quiet_mode {
            println!("Nothing to compare");
        }
    }
}
