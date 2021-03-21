use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{env, fs};
use tempfile::tempdir;

#[cfg(not(windows))]
use gag::Gag;

pub fn compare_benchmark(c: &mut Criterion) {
    let temp = tempdir().expect("create tempdir");
    let path = temp.into_path();

    let simple_compare_base_path = path.join("simple_compare_base.env");
    let simple_compare_other_path = path.join("simple_compare_other.env");

    let current_dir = env::current_dir().expect("get current dir");
    let app = dotenv_linter::cli::new(current_dir.as_os_str());
    let matches = app.get_matches_from(vec![
        "dotenv-linter",
        simple_compare_base_path
            .to_str()
            .expect("path conversion failed"),
        simple_compare_other_path
            .to_str()
            .expect("path conversion failed"),
    ]);

    fs::copy(
        "benches/fixtures/simple_compare_base.env",
        simple_compare_base_path,
    )
    .expect("copy .env file");
    fs::copy(
        "benches/fixtures/simple_compare_other.env",
        simple_compare_other_path,
    )
    .expect("copy .env file");

    c.bench_function("dotenv_linter compare", |b| {
        // Disable output to STDOUT
        #[cfg(not(windows))]
        let _print_gag = Gag::stdout().expect("disable stdout");

        b.iter(|| dotenv_linter::compare(black_box(&matches), black_box(&current_dir)))
    });
}

criterion_group!(benches, compare_benchmark);
criterion_main!(benches);
