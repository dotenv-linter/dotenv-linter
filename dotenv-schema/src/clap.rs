use clap::error::ErrorKind;

use crate::DotEnvSchema;

#[derive(Clone)]
pub struct DotEnvSchemaParser;

impl clap::builder::TypedValueParser for DotEnvSchemaParser {
    type Value = DotEnvSchema;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        match DotEnvSchema::load(value.to_string_lossy().as_ref()) {
            Ok(schema) => Ok(schema),
            Err(e) => Err(clap::Error::raw(
                ErrorKind::InvalidValue,
                format!("Failed to parse dotenv schema: {e}"),
            )),
        }
    }
}
