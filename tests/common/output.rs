const FIX_SENTENCE: &str = "All warnings are fixed. Total:";

pub fn fix_output(warnings: &[&str]) -> String {
    let total = warnings.len();
    let output: &str = &warnings.join("\n");
    String::from(format!("{}\n\n{} {}\n", output, FIX_SENTENCE, total))
}

pub fn check_output(warnings: &[&str]) -> String {
    let total = warnings.len();
    let output: &str = &warnings.join("\n");
    if total == 1 {
        return format!("{}\n\nFound {} problem\n", output, total);
    } else {
        return format!("{}\n\nFound {} problems\n", output, total);
    }
}
