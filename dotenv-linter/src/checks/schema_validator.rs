use std::collections::HashSet;

use super::Check;
use crate::{
    common::{LintKind, Warning},
    schema::SchemaValueType,
};
use dotenv_lookup::LineEntry;
use email_address::EmailAddress;
use url::Url;
pub(crate) struct SchemaChecker<'a> {
    schema: Option<&'a crate::schema::DotEnvSchema>,
    seen_keys: HashSet<String>,
    last_line_number: usize,
}

impl<'a> SchemaChecker<'a> {
    pub fn new(opts: &'a crate::cli::options::CliOptions) -> Self {
        Self {
            schema: opts.schema.as_ref(),
            seen_keys: HashSet::new(),
            last_line_number: 0,
        }
    }
}

impl Check for SchemaChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        self.schema?;
        self.last_line_number = line.number;
        let schema = self.schema.unwrap();
        let key = line.get_key()?;
        self.seen_keys.insert(key.to_string());

        if let Some(entry) = schema.entries.iter().find(|i| i.key == key) {
            if let Some(value) = line.get_value() {
                match entry.value_type {
                    SchemaValueType::String => {
                        if let Some(regex) = &entry.regex {
                            if !regex::Regex::new(regex).unwrap().is_match(value) {
                                return Some(Warning::new(
                                    line.number,
                                    self.name(),
                                    format!("The {} key does not match the regex", key),
                                ));
                            }
                        }
                    }
                    SchemaValueType::Boolean => {
                        if value == "true" || value == "false" {
                            return None;
                        }
                    }
                    SchemaValueType::Number => {
                        if value.parse::<i32>().is_err() {
                            return Some(Warning::new(
                                line.number,
                                self.name(),
                                format!("The {} key is not numeric", key),
                            ));
                        }
                    }
                    SchemaValueType::Email => {
                        if !EmailAddress::is_valid(value) {
                            return Some(Warning::new(
                                line.number,
                                self.name(),
                                format!("The {} key is not a valid email address", key),
                            ));
                        }
                    }
                    SchemaValueType::Url => {
                        if Url::parse(value).is_err() {
                            return Some(Warning::new(
                                line.number,
                                self.name(),
                                format!("The {} key is not a valid URL", key),
                            ));
                        }
                    }
                }
            } else {
                return Some(Warning::new(
                    line.number,
                    self.name(),
                    format!("The {} key is not defined in the schema", key),
                ));
            }
        } else if !schema.allow_other_keys {
            return Some(Warning::new(
                line.number,
                self.name(),
                format!("The {} key is not defined in the schema", key),
            ));
        }

        None
    }

    fn name(&self) -> LintKind {
        LintKind::SchemaViolation
    }
    fn end(&mut self) -> Vec<Warning> {
        let mut warnings = Vec::new();
        if let Some(schema) = self.schema {
            for entry in &schema.entries {
                if entry.required && !self.seen_keys.contains(&entry.key) {
                    warnings.push(Warning::new(
                        self.last_line_number,
                        self.name(),
                        format!("The {} key is required", entry.key),
                    ));
                }
            }
        }
        warnings
    }
}
