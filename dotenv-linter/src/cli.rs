use clap::{Arg, Command};

/// Build the CLI parser for dotenv-linter
pub fn build_cli() -> Command<'static> {
    Command::new("dotenv-linter")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Lint .env files for common mistakes")
        // existing positional “path” argument
        .arg(
            Arg::new("path")
                .help("Path(s) to .env file(s)")
                .multiple_occurrences(true)
                .takes_value(true),
        )
        // ← new: read .env content from stdin
        .arg(
            Arg::new("stdin")
                .long("stdin")
                .help("Read .env content from stdin"),
        )
        // ← new: supply a fake filename when reading from stdin
        .arg(
            Arg::new("stdin-filename")
                .long("stdin-filename")
                .takes_value(true)
                .help("Filename to show in diagnostics when reading from stdin"),
        )
}
