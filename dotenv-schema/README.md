# Dotenv-schema

[ci-badge]: https://github.com/dotenv-linter/dotenv-linter/workflows/CI/badge.svg
[ci-url]: https://github.com/dotenv-linter/dotenv-linter/actions
[crates-badge]: https://img.shields.io/crates/v/dotenv-schema
[crates-url]: https://crates.io/crates/dotenv-schema
[docs-badge]: https://img.shields.io/docsrs/dotenv-schema
[docs-url]: https://docs.rs/dotenv-schema
[codecov-url]: https://codecov.io/gh/mgrachev/dotenv-schema
[downloads-badge]: https://img.shields.io/crates/d/dotenv-schema
[MIT]: https://choosealicense.com/licenses/mit

[![CI][ci-badge]][ci-url]
[![Version][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![Downloads][downloads-badge]][crates-url]

A crate to validate `.env` files against schemas.

## Usage

Add `dotenv-schema` to `Cargo.toml`:

```toml
[dependencies]
dotenv-schema = "0.1"
```

## To validate `.env` file against schema

```rust
use dotenv_schema::{DotEnvSchema, ValidateResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dotenv_schema = r#"{
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
    let dotenv_schema_filename = ".dotenv_schema.json";

    std::fs::write(dotenv_schema_filename, dotenv_schema)?;

    let schema = match DotEnvSchema::load(dotenv_schema_filename) {
        Ok(schema) => Some(schema),
        Err(err) => {
            println!("Error loading schema: {err}");
            std::process::exit(1);
        }
    };

    let Some(schema) = schema else {
        return Err("Error loading .dotenv_schema.json".into());
    };

    let Some(port) = schema.entries.get("PORT") else {
        return Err("Error getting key PORT".into());
    };

    match port.is_valid("ABCD") {
        ValidateResult::Valid => {
            println!("PORT is valid");
        }
        ValidateResult::Invalid(_) => {
            println!("PORT is invalid");
        }
    }

    Ok(())
}
```

## MSRV

Minimum Supported Rust Version: 1.56.1

## License

[MIT]
