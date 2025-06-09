mod cli;
use cli::build_cli;
use dotenv_linter::run;

fn main() {
    let matches = build_cli().get_matches();
    if let Err(err) = run(&matches) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
