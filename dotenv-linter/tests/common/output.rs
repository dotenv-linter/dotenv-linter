enum Mode {
    Fix,
    Check,
}

/// Builds test output for validation.
///
/// # Arguments
///
/// * `blocks` - A slice of tuples, each containing a file path and a slice of
///   warnings for that file
/// * `mode` - Mode in which the program is run.
fn build_output(blocks: &[(&str, &[&str])], mode: Mode) -> String {
    let mut output = String::new();
    let mut count = 0;

    if blocks.is_empty() {
        return String::new();
    }

    for (i, b) in blocks.iter().enumerate() {
        // Print using `b.0` which is the file path
        match mode {
            Mode::Fix => {
                output.push_str(&format!("Fixing {}\n", b.0));
            }
            Mode::Check => {
                output.push_str(&format!("Checking {}\n", b.0));
            }
        }
        if !b.1.is_empty() {
            // Print using `b.1` which is a slice of warnings
            for w in b.1 {
                count += 1;
                output.push_str(&format!("{}\n", w));
            }
            if i != blocks.len() - 1 {
                output.push('\n');
            }
        }
    }

    match mode {
        Mode::Fix => {
            if count > 0 {
                output.push_str(&format!("\nAll warnings are fixed. Total: {}\n", count));
            } else {
                output.push_str("\nNo warnings found\n");
            }
        }
        Mode::Check => {
            if count == 0 {
                output.push_str("\nNo problems found\n");
            } else if count == 1 {
                output.push_str(&format!("\nFound {} problem\n", count));
            } else {
                output.push_str(&format!("\nFound {} problems\n", count));
            }
        }
    }

    output
}

/// Builds test output for fixes.
///
/// # Arguments
///
/// * `blocks` - A slice of tuples, each containing a file path and a slice of
///   warnings for that file
pub fn fix_output(blocks: &[(&str, &[&str])]) -> String {
    build_output(blocks, Mode::Fix)
}

/// Builds test output for checks.
///
/// # Arguments
///
/// * `blocks` - A slice of tuples, each containing a file path and a slice of
///   warnings for that file
pub fn check_output(blocks: &[(&str, &[&str])]) -> String {
    build_output(blocks, Mode::Check)
}
