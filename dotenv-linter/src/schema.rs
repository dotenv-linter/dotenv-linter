use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

// the Readxxx structs are used to deserialize via serde
// they are then post processed into the xxx structs

#[derive(Deserialize, Default)]
#[serde(default)]
struct ReadDotEnvSchema {
    pub version: String,
    pub allow_other_keys: bool,
    pub entries: Vec<ReadSchemaEntry>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct ReadSchemaEntry {
    pub key: String,
    pub required: bool,
    #[serde(rename = "type")]
    pub value_type: SchemaValueType,
    pub regex: Option<String>,
}

pub struct DotEnvSchema {
    pub version: String,
    pub allow_other_keys: bool,
    pub entries: HashMap<String, SchemaEntry>,
}

pub struct SchemaEntry {
    pub key: String,
    pub required: bool,
    pub value_type: SchemaValueType,
    pub regex: Option<Regex>,
}

impl SchemaEntry {
    pub fn new(read_entry: ReadSchemaEntry) -> Result<Self, regex::Error> {
        let mut se = SchemaEntry {
            key: read_entry.key,
            required: read_entry.required,
            value_type: read_entry.value_type,
            regex: None,
        };
        if let Some(reg) = read_entry.regex {
            se.regex = Some(Regex::new(&reg)?);
        }
        Ok(se)
    }
}

#[derive(Deserialize, Default)]
pub enum SchemaValueType {
    #[default]
    String,
    Integer,
    Float,
    Boolean,
    Url,
    Email,
}

impl DotEnvSchema {
    pub fn load(path: &Path) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let read_schema: ReadDotEnvSchema = serde_json::from_reader(reader)?;
        let mut schema = DotEnvSchema {
            version: read_schema.version,
            allow_other_keys: read_schema.allow_other_keys,
            entries: HashMap::new(),
        };

        for e in read_schema.entries {
            let se = SchemaEntry::new(e)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            schema.entries.insert(se.key.clone(), se);
        }
        Ok(schema)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::Write;

    use dotenv_lookup::LineEntry;
    use regex::Regex;
    use tempfile::tempdir;

    use super::{DotEnvSchema, ReadDotEnvSchema, SchemaEntry};

    use crate::cli::options::CliOptions;
    use crate::common::tests::*;
    use crate::{LintKind, Warning};

    fn load_schema() -> Result<DotEnvSchema, std::io::Error> {
        let json = r#"{
            "version": "1.0.0",
            "entries": [
                {
                    "key": "NAME",
                    "type": "String"
                },
                {
                    "key": "PORT",
                    "type": "Integer"
                },
                {
                    "key": "PRICE",
                    "type": "Float"
                },
                {
                    "key": "URL",
                    "type": "Url"
                },
                {
                    "key": "EMAIL",
                    "type": "Email"
                },
                {
                    "key": "FLAG",
                    "type": "Boolean"
                }
            ]
        }"#;
        let read_schema: ReadDotEnvSchema = serde_json::from_str(json).unwrap();
        let mut schema = DotEnvSchema {
            version: read_schema.version,
            allow_other_keys: read_schema.allow_other_keys,
            entries: HashMap::new(),
        };

        for e in read_schema.entries {
            let se = SchemaEntry::new(e)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            schema.entries.insert(se.key.clone(), se);
        }
        Ok(schema)
    }

    #[test]
    fn string_good() {
        let schema = load_schema().expect("failed to load schema");
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "NAME=joe")];
        let expected: Vec<Warning> = Vec::new();
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn string_unknown() {
        let schema = load_schema().expect("failed to load schema");
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
        let mut schema = load_schema().expect("failed to load schema");
        schema.allow_other_keys = true;
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "USER=joe")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn integer_good() {
        let schema = load_schema().expect("failed to load schema");
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "PORT=42")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn integer_bad() {
        let schema = load_schema().expect("failed to load schema");
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "PORT=p")];
        let expected: Vec<Warning> = vec![Warning::new(
            1,
            LintKind::SchemaViolation,
            "The PORT key is not an integer",
        )];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn integer_is_float() {
        let schema = load_schema().expect("failed to load schema");
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "PORT=2.4")];
        let expected: Vec<Warning> = vec![Warning::new(
            1,
            LintKind::SchemaViolation,
            "The PORT key is not an integer",
        )];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn float_good() {
        let schema = load_schema().expect("failed to load schema");
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "PRICE=2.4")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn float_good2() {
        let schema = load_schema().expect("failed to load schema");
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "PRICE=24")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn float_bad() {
        let schema = load_schema().expect("failed to load schema");
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "PRICE=price")];
        let expected: Vec<Warning> = vec![Warning::new(
            1,
            LintKind::SchemaViolation,
            "The PRICE key is not a valid float",
        )];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn url_good() {
        let schema = load_schema().expect("failed to load schema");
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "URL=https://example.com")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn url_bad() {
        let schema = load_schema().expect("failed to load schema");
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
        let schema = load_schema().expect("failed to load schema");
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "EMAIL=joe@gmail.com")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn email_bad() {
        let schema = load_schema().expect("failed to load schema");
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
        let mut schema = load_schema().expect("failed to load schema");
        schema.entries.get_mut("EMAIL").unwrap().required = true;
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "EMAIL=joe@gmail.com")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn required_missing() {
        let mut schema = load_schema().expect("failed to load schema");
        schema.entries.get_mut("EMAIL").unwrap().required = true;
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
        let mut schema = load_schema().expect("failed to load schema");
        schema.entries.get_mut("NAME").unwrap().regex =
            Some(Regex::new("^[ABCD]*$").expect("Bad regex"));
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "NAME=BAD")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn regex_bad() {
        let mut schema = load_schema().expect("failed to load schema");
        schema.entries.get_mut("NAME").unwrap().regex =
            Some(Regex::new("^[ABCD]*$").expect("Bad regex"));
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

    #[test]
    fn boolean_good() {
        let schema = load_schema().expect("failed to load schema");
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "FLAG=true")];
        let expected: Vec<Warning> = vec![];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn boolean_bad() {
        let schema = load_schema().expect("failed to load schema");
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "FLAG=joe")];
        let expected: Vec<Warning> = vec![Warning::new(
            1,
            LintKind::SchemaViolation,
            "The FLAG key is not a valid boolean",
        )];
        let mut opt = CliOptions::default();
        opt.schema = Some(schema);
        assert_eq!(expected, crate::checks::run(&lines, &opt));
    }

    #[test]
    fn create_file_schema() {
        let json = r#"{
            "version": "1.0.0",
            "entries": [
                {
                    "key": "NAME",
                    "type": "String",
                    "regex": "^[ABCD]*$"
                },
                {
                    "key": "PORT",
                    "type": "Integer"
                },
                {
                    "key": "URL",
                    "type": "Url"
                },
                {
                    "key": "EMAIL",
                    "type": "Email"
                },
                {
                    "key": "FLAG",
                    "type": "Boolean"
                }
            ]
        }"#;
        // write the above json to a temp file
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("schema.json");
        let schema = {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(json.as_bytes()).unwrap();
            // load the schema from the file
            DotEnvSchema::load(&file_path)
        };
        fs::remove_file(&file_path).unwrap();
        assert!(schema.is_ok());
    }

    #[test]
    fn load_missing_file() {
        assert!(DotEnvSchema::load(std::path::Path::new("bad_file.json")).is_err());
    }

    #[test]
    fn create_bad_regex_file_schema() {
        let json = r#"{
            "version": "1.0.0",
            "entries": [
                {
                    "key": "NAME",
                    "type": "String",
                    "regex": "^[ABCD*$"
                },
                {
                    "key": "PORT",
                    "type": "Integer"
                },
                {
                    "key": "URL",
                    "type": "Url"
                },
                {
                    "key": "EMAIL",
                    "type": "Email"
                },
                {
                    "key": "FLAG",
                    "type": "Boolean"
                }
            ]
        }"#;
        // write the above json to a temp file
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("schema.json");
        let schema = {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(json.as_bytes()).unwrap();
            // load the schema from the file
            DotEnvSchema::load(&file_path)
        };
        fs::remove_file(&file_path).unwrap();
        assert!(schema.is_err());
    }

    #[test]
    fn create_bad_file_schema() {
        let json = r#"{
            "version": "1.0.0",
            bad:json
        }"#;
        // write the above json to a temp file
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("schema.json");
        let schema = {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(json.as_bytes()).unwrap();
            // load the schema from the file
            DotEnvSchema::load(&file_path)
        };
        fs::remove_file(&file_path).unwrap();
        assert!(schema.is_err());
    }
}
