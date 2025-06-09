use clap::{Arg, ArgAction, Command};

/// Build the CLI parser for dotenv-linter
pub fn build_cli() -> Command {
    Command::new("dotenv-linter")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Lint .env files for common mistakes")
        // zero or more PATH values; defaults to “.env” if none given
        .arg(
            Arg::new("path")
                .help("Path(s) to .env file(s)")
                .num_args(0..)
                .value_name("PATH"),
        )
        // flag: read from stdin
        .arg(
            Arg::new("stdin")
                .long("stdin")
                .help("Read .env content from stdin")
                .action(ArgAction::SetTrue),
        )
        // one value: fake filename for diagnostics
        .arg(
            Arg::new("stdin-filename")
                .long("stdin-filename")
                .help("Filename to show in diagnostics when reading from stdin")
                .num_args(1)
                .value_name("FILENAME"),
        )
}
