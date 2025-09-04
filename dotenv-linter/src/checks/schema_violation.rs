use std::collections::HashSet;

use dotenv_core::LineEntry;
use dotenv_schema::{DotEnvSchema, SchemaValueType, ValidateResult};

use super::Check;
use crate::common::{LintKind, Warning};

pub(crate) struct SchemaViolationChecker<'a> {
    schema: Option<&'a DotEnvSchema>,
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
        let value = line.get_value()?;

        let Some(entry) = schema.entries.get(key) else {
            if schema.allow_other_keys {
                return None;
            }

            return Some(Warning::new(
                line.number,
                self.name(),
                format!("The {key} key is not defined in the schema"),
            ));
        };

        let ValidateResult::Invalid(ty) = entry.is_valid(value) else {
            return None;
        };

        let message = match ty {
            SchemaValueType::String => {
                format!("The {key} key does not match the regex")
            }
            SchemaValueType::Integer => {
                format!("The {key} key is not an integer")
            }
            SchemaValueType::Float => {
                format!("The {key} key is not a valid float")
            }
            SchemaValueType::Boolean => {
                format!("The {key} key is not a valid boolean")
            }
            SchemaValueType::Url => {
                format!("The {key} key is not a valid URL")
            }
            SchemaValueType::Email => {
                format!("The {key} key is not a valid email address")
            }
        };

        Some(Warning::new(line.number, self.name(), message))
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
