use dotenv_core::LineEntry;

#[test]
fn normal_analyzer_fix_pipeline_keeps_export_behavior() {
    let mut lines = vec![
        LineEntry::new(1, "export foo=bar", false),
        LineEntry::new(2, "", true),
    ];
    let warnings = dotenv_analyzer::check(&lines, &[], None);
    assert!(!warnings.is_empty());
    assert!(dotenv_analyzer::fix(&warnings, &mut lines, &[]) > 0);
    assert!(lines[0].raw_string.starts_with("export "));
    assert_eq!(lines[0].get_key(), Some("FOO"));
}
