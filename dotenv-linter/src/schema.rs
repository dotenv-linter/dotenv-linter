use anyhow::Result;
use serde::Deserialize;
use std::{fs::File, io::BufReader, path::Path};
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct DotEnvSchema {
    pub version: String,
    pub allow_other_keys: bool,
    pub entries: Vec<SchemaEntry>,
}
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct SchemaEntry {
    pub key: String,
    pub required: bool,
    #[serde(rename = "type")]
    pub value_type: SchemaValueType,
    pub regex: Option<String>,
}
#[derive(Deserialize, Default)]

pub enum SchemaValueType {
    #[default]
    String,
    Number,
    Boolean,
    Url,
    Email,
}
impl DotEnvSchema {
    pub fn load(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let schema: Self = serde_json::from_reader(reader)?;
        Ok(schema)
    }
}
#[cfg(test)]
mod tests {
    use dotenv_lookup::LineEntry;

    use super::DotEnvSchema;

    use crate::cli::options::CliOptions;
    use crate::common::tests::*;
    use crate::{LintKind, Warning};

    fn load_schema() -> DotEnvSchema {
        let json = r#"{
            "version": "1.0.0",
            "entries": [
                {
                    "key": "NAME",
                    "type": "String"
                },
                {
                    "key": "PORT",
                    "type": "Number"
                },
                {
                    "key": "URL",
                    "type": "Url"
                },
                {
                    "key": "EMAIL",
                    "type": "Email"
                }
            ]
        }"#;
        let schema: DotEnvSchema = serde_json::from_str(json).unwrap();
        schema
    }
    #[test]
    fn string_good() {
        let schema = load_schema();
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "NAME=joe")];
        let expected: Vec<Warning> = Vec::new();
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }
    #[test]
    fn string_unknown() {
        let schema = load_schema();
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "USER=joe")];
        let expected: Vec<Warning> = vec![Warning::new(
            1,
            LintKind::SchemaViolation,
            "The USER key is not defined in the schema",
        )];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }
    #[test]
    fn string_unknown_allowed() {
        let mut schema = load_schema();
        schema.allow_other_keys = true;
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "USER=joe")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }
    #[test]
    fn numeric_good() {
        let schema = load_schema();
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "PORT=42")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }
    #[test]
    fn numeric_bad() {
        let schema = load_schema();
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "PORT=p")];
        let expected: Vec<Warning> = vec![Warning::new(
            1,
            LintKind::SchemaViolation,
            "The PORT key is not numeric",
        )];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }
    #[test]
    fn url_good() {
        let schema = load_schema();
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "URL=https://example.com")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }
    #[test]
    fn url_bad() {
        let schema = load_schema();
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "URL=not_a_url")];
        let expected: Vec<Warning> = vec![Warning::new(
            1,
            LintKind::SchemaViolation,
            "The URL key is not a valid URL",
        )];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }
    #[test]
    fn email_good() {
        let schema = load_schema();
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "EMAIL=joe@gmail.com")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }
    #[test]
    fn email_bad() {
        let schema = load_schema();
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "EMAIL=not_an_eamil")];
        let expected: Vec<Warning> = vec![Warning::new(
            1,
            LintKind::SchemaViolation,
            "The EMAIL key is not a valid email address",
        )];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }
    #[test]
    fn required_present() {
        let mut schema = load_schema();
        schema.entries[3].required = true;
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "EMAIL=joe@gmail.com")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }
    #[test]
    fn required_missing() {
        let mut schema = load_schema();
        schema.entries[3].required = true;
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "NAME=joe")];
        let expected: Vec<Warning> = vec![Warning::new(
            1,
            LintKind::SchemaViolation,
            "The EMAIL key is required",
        )];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }
    #[test]
    fn regex_good() {
        let mut schema = load_schema();
        schema.entries[0].regex = Some("^[ABCD]*$".to_string());
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "NAME=BAD")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }
    #[test]
    fn regex_bad() {
        let mut schema = load_schema();
        schema.entries[0].regex = Some("^[ABCD]*$".to_string());
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "NAME=joe")];
        let expected: Vec<Warning> = vec![Warning::new(
            1,
            LintKind::SchemaViolation,
            "The NAME key does not match the regex",
        )];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }
}
