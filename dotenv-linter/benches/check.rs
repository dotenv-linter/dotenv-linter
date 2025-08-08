use std::{env, fs, hint::black_box};

use criterion::{criterion_group, criterion_main, Criterion};
#[cfg(not(windows))]
use gag::Gag;
use tempfile::tempdir;

pub fn check_benchmark(c: &mut Criterion) {
    let temp = tempdir().expect("create tempdir");
    let path = temp.keep();

    let current_dir = env::current_dir().expect("get current dir");
    let app = dotenv_linter::cli::command();
    let args = app.get_matches_from(vec!["dotenv-linter", path.to_str().expect("path to str")]);
    let opts = dotenv_linter::cli::options::CheckOptions::new(&args);

    fs::copy("benches/fixtures/simple.env", path.join(".env")).expect("copy .env file");

    c.bench_function("dotenv_linter check", |b| {
        // Disable output to STDOUT
        #[cfg(not(windows))]
        let _print_gag = Gag::stdout().expect("disable stdout");

        b.iter(|| dotenv_linter::check(black_box(&opts), black_box(&current_dir)))
    });
}

criterion_group!(benches, check_benchmark);
criterion_main!(benches);
