use crate::common::{FileEntry, Warning};
use colored::*;
use std::ffi::OsString;

/// Mode in which the program is run.
#[derive(Clone, Copy)]
pub enum Mode {
    Fix,
    Check,
}

/// Prefix for the backup output.
const BACKUP_PREFIX: &str = "Original file was backed up to: ";

/// Wraps warnings to provide more information when printing.
#[derive(Clone, Copy)]
pub struct Output {
    /// Mode of the program.
    mode: Mode,

    // Quiet program output mode
    is_quiet_mode: bool,
}

impl Output {
    pub fn new(is_fix_mode: bool, is_quiet_mode: bool) -> Self {
        let mode = if is_fix_mode { Mode::Fix } else { Mode::Check };

        Self {
            mode,
            is_quiet_mode,
        }
    }

    /// Prints information about a file in process
    pub fn print_processing_info(self, file: &FileEntry) {
        match self.mode {
            Mode::Fix if !&self.is_quiet_mode => {
                println!("Fixing {}", file);
            }
            Mode::Check if !&self.is_quiet_mode => {
                println!("Checking {}", file);
            }
            _ => {}
        }
    }

    /// Prints warnings without any additional information.
    pub fn print_warnings(self, warnings: &[Warning], is_last_file: bool) {
        match self.mode {
            Mode::Fix if self.is_quiet_mode => {}
            _ => {
                warnings.iter().for_each(|w| println!("{}", w));
                if !warnings.is_empty() && !is_last_file {
                    println!();
                }
            }
        }
    }

    /// Prints the backup file's path.
    pub fn print_backup(self, backup_path: &OsString) {
        println!("{}{:?}", BACKUP_PREFIX, backup_path);
        if !self.is_quiet_mode {
            println!();
        }
    }

    pub fn print_total(self, total: usize) {
        match self.mode {
            Mode::Fix => {
                Output::print_fix_total(total);
            }
            Mode::Check if !self.is_quiet_mode => {
                Output::print_check_total(total);
            }
            _ => {}
        }
    }

    fn print_fix_total(total: usize) {
        if total != 0 {
            println!("\nAll warnings are fixed. Total: {}", total);
        } else {
            println!("\nNo warnings found");
        }
    }

    fn print_check_total(total: usize) {
        if total != 0 {
            let mut problems = String::from("problem");

            if total != 1 {
                problems += "s";
            }

            println!(
                "\n{}",
                format!(
                    "{} {} {}",
                    String::from("Found"),
                    total.to_string(),
                    problems
                )
                .red()
                .bold()
            );
        } else {
            println!("\n{}", "No problems found".to_string().green().bold());
        }
    }
}
