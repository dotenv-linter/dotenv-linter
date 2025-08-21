use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct SchemaEntry {
    pub key: String,
    pub required: bool,
    #[serde(rename = "type")]
    pub value_type: SchemaValueType,
    #[serde(with = "serde_regex")]
    pub regex: Option<Regex>,
}
#[derive(Deserialize, Default, Debug)]
pub struct DotEnvSchema {
    pub version: String,
    #[serde(default)]
    pub allow_other_keys: bool,
    #[serde(with = "::serde_with::rust::maps_duplicate_key_is_error")]
    pub entries: HashMap<String, SchemaEntry>,
}

#[derive(Deserialize, Default, Debug)]
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
        let read_schema: DotEnvSchema = serde_json::from_reader(reader)?;
        Ok(read_schema)
    }
}

pub enum ValidateResult {
    Valid,
    Invalid(SchemaValueType),
}

impl SchemaEntry {
    pub fn is_valid(&self, value: &str) -> ValidateResult {
        match self.value_type {
            SchemaValueType::String => {
                let Some(regex) = &self.regex else {
                    return ValidateResult::Valid;
                };

                if !regex.is_match(value) {
                    return ValidateResult::Invalid(SchemaValueType::String);
                };
            }
            SchemaValueType::Boolean => {
                if !matches!(
                    value,
                    "true" | "false" | "TRUE" | "FALSE" | "yes" | "no" | "YES" | "NO" | "1" | "0"
                ) {
                    return ValidateResult::Invalid(SchemaValueType::Boolean);
                }
            }
            SchemaValueType::Integer => {
                if value.parse::<i32>().is_err() {
                    return ValidateResult::Invalid(SchemaValueType::Integer);
                }
            }
            SchemaValueType::Float => {
                if value.parse::<f32>().is_err() {
                    return ValidateResult::Invalid(SchemaValueType::Float);
                }
            }
            SchemaValueType::Email => {
                if !email_address::EmailAddress::is_valid(value) {
                    return ValidateResult::Invalid(SchemaValueType::Email);
                }
            }
            SchemaValueType::Url => {
                if url::Url::parse(value).is_err() {
                    return ValidateResult::Invalid(SchemaValueType::Url);
                }
            }
        }

        ValidateResult::Valid
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::Write,
    };

    use tempfile::tempdir;

    use super::DotEnvSchema;

    #[test]
    fn create_file_schema() {
        let json = r#"{
            "version": "1.0.0",
            "entries": {
                "NAME": {
                    "type": "String"
                },
                "PORT": {
                    "type": "Integer"
                },
                "PRICE": {
                    "type": "Float"
                },
                "URL": {
                    "type": "Url"
                },
                "EMAIL":{
                    "type": "Email"
                },
                "FLAG":{
                    "type": "Boolean"
                }
            }
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
            "entries": {
                "NAME": {
                    "type": "String",
                    "regex": "~[.."
                },
                "PORT": {
                    "type": "Integer"
                },
                "PRICE": {
                    "type": "Float"
                },
                "URL": {
                    "type": "Url"
                },
                "EMAIL":{
                    "type": "Email"
                },
                "FLAG":{
                    "type": "Boolean"
                }
            }
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
    #[test]
    fn test_dup_schema() {
        let json = r#"{
            "version": "1.0.0",
            "entries": {
                "NAME": {
                    "type": "String"
                },
                "NAME": {
                    "type": "String"
                },
                "PORT": {
                    "type": "Integer"
                },
                "PRICE": {
                    "type": "Float"
                },
                "URL": {
                    "type": "Url"
                },
                "EMAIL":{
                    "type": "Email"
                },
                "FLAG":{
                    "type": "Boolean"
                }
            }
        }"#;
        let schema: serde_json::Result<DotEnvSchema> = serde_json::from_str(json);
        assert!(schema.is_err());
        assert_eq!(
            schema.unwrap_err().to_string(),
            "invalid entry: found duplicate key at line 9 column 17"
        );
    }
}
