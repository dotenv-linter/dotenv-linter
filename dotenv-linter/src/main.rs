use clap::{Arg, Command};
use dotenv_linter::run;

fn build_cli() -> Command<'static> {
    Command::new("dotenv-linter")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Lint .env files for common mistakes")
        // existing file‐path argument
        .arg(
            Arg::new("path")
                .help("Path(s) to .env file(s)")
                .multiple_occurrences(true)
                .takes_value(true),
        )
        // ← new: read from stdin
        .arg(
            Arg::new("stdin")
                .long("stdin")
                .help("Read .env content from stdin"),
        )
        // ← new: set filename for diagnostics
        .arg(
            Arg::new("stdin-filename")
                .long("stdin-filename")
                .takes_value(true)
                .help("Filename to show in diagnostics when reading from stdin"),
        )
}

fn main() {
    let matches = build_cli().get_matches();
    if let Err(err) = run(&matches) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
