use std::{env, fs, hint::black_box};

use criterion::{Criterion, criterion_group, criterion_main};
#[cfg(not(windows))]
use gag::Gag;
use tempfile::tempdir;

pub fn check_benchmark(c: &mut Criterion) {
    let temp = tempdir().expect("create tempdir");
    let path = temp.keep();

    let current_dir = env::current_dir().expect("get current dir");
    let opts = dotenv_linter::CheckOptions {
        files: vec![&path],
        ignore_checks: vec![],
        exclude: vec![],
        quiet: false,
        recursive: false,
        schema: None,
    };

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
