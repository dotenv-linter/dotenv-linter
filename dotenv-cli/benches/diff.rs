use std::{env, fs, hint::black_box};

use criterion::{Criterion, criterion_group, criterion_main};
#[cfg(not(windows))]
use gag::Gag;
use tempfile::tempdir;

pub fn diff_benchmark(c: &mut Criterion) {
    let temp = tempdir().expect("create tempdir");
    let path = temp.keep();

    let current_dir = env::current_dir().expect("get current dir");
    let env = path.join(".env");
    let env_compare = path.join(".env.compare");
    let opts = dotenv_linter::DiffOptions {
        files: vec![&env, &env_compare],
        quiet: false,
    };

    fs::copy("benches/fixtures/simple.env", path.join(".env")).expect("copy .env file");
    fs::copy("benches/fixtures/compare.env", path.join(".env.compare"))
        .expect("copy .env.compare file");

    c.bench_function("dotenv_linter compare", |b| {
        // Disable output to STDOUT
        #[cfg(not(windows))]
        let _print_gag = Gag::stdout().expect("disable stdout");

        b.iter(|| dotenv_linter::diff(black_box(&opts), black_box(&current_dir)))
    });
}

criterion_group!(benches, diff_benchmark);
criterion_main!(benches);
