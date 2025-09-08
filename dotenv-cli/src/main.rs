use dotenv_linter::{Result, cli};

fn main() -> Result<()> {
    let code = cli::run()?;
    std::process::exit(code);
}
