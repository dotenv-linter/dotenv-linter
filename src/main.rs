use dotenv_linter::{cli, Result};

fn main() -> Result<()> {
    let code = cli::run()?;
    std::process::exit(code);
}
