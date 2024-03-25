use std::collections::HashSet;

use super::Check;
use crate::{
    common::{LintKind, Warning},
    schema::{DotEnvSchema, SchemaValueType},
};
use dotenv_lookup::LineEntry;
use email_address::EmailAddress;
use url::Url;
pub(crate) struct SchemaViolationChecker<'a> {
    schema: Option<&'a crate::schema::DotEnvSchema>,
    seen_keys: HashSet<String>,
    last_line_number: usize,
}

impl<'a> SchemaViolationChecker<'a> {
    pub fn new(schema: Option<&'a DotEnvSchema>) -> Self {
        Self {
            schema: schema,
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
                                    format!("The {} key does not match the regex", key),
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
                                format!("The {} key is not a valid boolean", key),
                            ));
                        }
                    }
                    SchemaValueType::Integer => {
                        if value.parse::<i32>().is_err() {
                            return Some(Warning::new(
                                line.number,
                                self.name(),
                                format!("The {} key is not an integer", key),
                            ));
                        }
                    }
                    SchemaValueType::Float => {
                        if value.parse::<f32>().is_err() {
                            return Some(Warning::new(
                                line.number,
                                self.name(),
                                format!("The {} key is not a valid float", key),
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
            for (key, entry) in &schema.entries {
                if entry.required && !self.seen_keys.contains(key) {
                    warnings.push(Warning::new(
                        self.last_line_number,
                        self.name(),
                        format!("The {} key is required", key),
                    ));
                }
            }
        }
        warnings
    }
}
