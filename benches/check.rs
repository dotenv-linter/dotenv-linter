use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{env, fs};
use tempfile::tempdir;

#[cfg(not(windows))]
use gag::Gag;

pub fn check_benchmark(c: &mut Criterion) {
    let temp = tempdir().expect("create tempdir");
    let path = temp.into_path();

    let current_dir = env::current_dir().expect("get current dir");
    let app = dotenv_linter::cli::command();
    let matches = app.get_matches_from(vec!["dotenv-linter", path.to_str().expect("path to str")]);

    fs::copy("benches/fixtures/simple.env", path.join(".env")).expect("copy .env file");

    c.bench_function("dotenv_linter check", |b| {
        // Disable output to STDOUT
        #[cfg(not(windows))]
        let _print_gag = Gag::stdout().expect("disable stdout");

        b.iter(|| dotenv_linter::check(black_box(&matches), black_box(&current_dir)))
    });
}

criterion_group!(benches, check_benchmark);
criterion_main!(benches);
