const FIX_SENTENCE: &str = "All warnings are fixed. Total:";

pub fn fix_output(warnings: &[&str]) -> String {
    let total = warnings.len();
    let output: &str = &warnings.join("\n");
    format!("{}\n\n{} {}\n", output, FIX_SENTENCE, total)
}

pub fn check_output(warnings: &[&str]) -> String {
    let total = warnings.len();
    let output: &str = &warnings.join("\n");
    let mut problems = String::from("problem");
    if total > 1 {
        problems += "s";
    }
    format!("{}\n\nFound {} {}\n", output, total, problems)
}
