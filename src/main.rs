use std::error::Error;
use std::{env, process};

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    let current_dir = env::current_dir()?;
    let args = dotenv_linter::cli::new(current_dir.as_os_str()).get_matches();

    disable_color_output(&args);

    match args.subcommand() {
        ("", None) => {
            let total_warnings = dotenv_linter::check(&args, &current_dir)?;

            if total_warnings == 0 {
                process::exit(0);
            }
        }
        ("fix", Some(fix_args)) => {
            dotenv_linter::fix(&fix_args, &current_dir)?;
            process::exit(0);
        }
        ("list", Some(_)) => {
            dotenv_linter::available_check_names()
                .iter()
                .for_each(|name| println!("{}", name));

            process::exit(0);
        }
        ("compare", Some(compare_args)) => {
            disable_color_output(&compare_args);

            let warnings = dotenv_linter::compare(&compare_args, &current_dir)?;
            if warnings.is_empty() {
                process::exit(0);
            }
        }
        _ => {
            eprintln!("unknown command");
        }
    }

    process::exit(1);
}

fn disable_color_output(args: &clap::ArgMatches) {
    if args.is_present("no-color") {
        colored::control::set_override(false);
    }
}
