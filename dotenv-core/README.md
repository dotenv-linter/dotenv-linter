# Dotenv-core

[ci-badge]: https://github.com/dotenv-linter/dotenv-linter/workflows/CI/badge.svg
[ci-url]: https://github.com/dotenv-linter/dotenv-linter/actions
[crates-badge]: https://img.shields.io/crates/v/dotenv-core
[crates-url]: https://crates.io/crates/dotenv-core
[docs-badge]: https://img.shields.io/docsrs/dotenv-core
[docs-url]: https://docs.rs/dotenv-core
[codecov-url]: https://codecov.io/gh/mgrachev/dotenv-core
[downloads-badge]: https://img.shields.io/crates/d/dotenv-analyzer
[MIT]: https://choosealicense.com/licenses/mit

[![CI][ci-badge]][ci-url]
[![Version][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![Downloads][downloads-badge]][crates-url]

Common crate for `dotenv-linter` crates.

## Usage

Add `dotenv-core` and other dependencies to `Cargo.toml`:

```toml
[dependencies]
dotenv-analyzer = "0.1"
dotenv-core = "0.1"
dotenv-finder = "0.1"
```

## To check `.env` files

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;

    let files = dotenv_finder::FinderBuilder::new(&current_dir)
        .build()
        .find();

    for (file, lines) in files {
        let warnings = dotenv_analyzer::check(&lines, &[], None);

        for warning in warnings {
            println!(
                "Warning {file}:{}:{}",
                warning.check_name(),
                warning.message()
            );
        }
    }

    Ok(())
}
```

## To fix `.env` files

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;

    let files = dotenv_finder::FinderBuilder::new(&current_dir)
        .build()
        .find();

    for (_, mut lines) in files {
        let warnings = dotenv_analyzer::check(&lines, &[], None);

        dotenv_analyzer::fix(&warnings, &mut lines, &[]);
    }

    Ok(())
}
```

## MSRV

Minimum Supported Rust Version: 1.56.1

## License

[MIT]
