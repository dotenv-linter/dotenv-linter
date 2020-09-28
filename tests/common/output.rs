const FIX_SENTENCE: &str = "Fixed warnings:";

pub fn fix_output(warnings: &[&str]) -> String {
    let output: &str = &warnings.join("\n");
    String::from(format!("{}\n{}\n", FIX_SENTENCE, output))
}
