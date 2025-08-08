use std::collections::HashSet;

use dotenv_lookup::LineEntry;
use email_address::EmailAddress;
use url::Url;

use super::Check;
use crate::{
    common::{LintKind, Warning},
    schema::{DotEnvSchema, SchemaValueType},
};

pub(crate) struct SchemaViolationChecker<'a> {
    schema: Option<&'a crate::schema::DotEnvSchema>,
    seen_keys: HashSet<String>,
    last_line_number: usize,
}

impl<'a> SchemaViolationChecker<'a> {
    pub fn new(schema: Option<&'a DotEnvSchema>) -> Self {
        Self {
            schema,
            seen_keys: HashSet::new(),
            last_line_number: 0,
        }
    }
}

impl Check for SchemaViolationChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let schema = self.schema?;
        self.last_line_number = line.number;
        let key = line.get_key()?;
        self.seen_keys.insert(key.to_string());

        if let Some(entry) = schema.entries.get(key) {
            if let Some(value) = line.get_value() {
                match entry.value_type {
                    SchemaValueType::String => {
                        if let Some(regex) = &entry.regex {
                            if !regex.is_match(value) {
                                return Some(Warning::new(
                                    line.number,
                                    self.name(),
                                    format!("The {key} key does not match the regex"),
                                ));
                            }
                        }
                    }
                    SchemaValueType::Boolean => {
                        if matches!(
                            value,
                            "true"
                                | "false"
                                | "TRUE"
                                | "FALSE"
                                | "yes"
                                | "no"
                                | "YES"
                                | "NO"
                                | "1"
                                | "0"
                        ) {
                            return None;
                        } else {
                            return Some(Warning::new(
                                line.number,
                                self.name(),
                                format!("The {key} key is not a valid boolean"),
                            ));
                        }
                    }
                    SchemaValueType::Integer => {
                        if value.parse::<i32>().is_err() {
                            return Some(Warning::new(
                                line.number,
                                self.name(),
                                format!("The {key} key is not an integer"),
                            ));
                        }
                    }
                    SchemaValueType::Float => {
                        if value.parse::<f32>().is_err() {
                            return Some(Warning::new(
                                line.number,
                                self.name(),
                                format!("The {key} key is not a valid float"),
                            ));
                        }
                    }
                    SchemaValueType::Email => {
                        if !EmailAddress::is_valid(value) {
                            return Some(Warning::new(
                                line.number,
                                self.name(),
                                format!("The {key} key is not a valid email address"),
                            ));
                        }
                    }
                    SchemaValueType::Url => {
                        if Url::parse(value).is_err() {
                            return Some(Warning::new(
                                line.number,
                                self.name(),
                                format!("The {key} key is not a valid URL"),
                            ));
                        }
                    }
                }
            }
        } else if !schema.allow_other_keys {
            return Some(Warning::new(
                line.number,
                self.name(),
                format!("The {key} key is not defined in the schema"),
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
            for (key, entry) in &schema.entries {
                if entry.required && !self.seen_keys.contains(key) {
                    warnings.push(Warning::new(
                        self.last_line_number,
                        self.name(),
                        format!("The {key} key is required"),
                    ));
                }
            }
        }
        warnings
    }
}
