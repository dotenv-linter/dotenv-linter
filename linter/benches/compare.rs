use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{env, fs};
use tempfile::tempdir;

#[cfg(not(windows))]
use gag::Gag;

pub fn compare_benchmark(c: &mut Criterion) {
    let temp = tempdir().expect("create tempdir");
    let path = temp.into_path();

    let current_dir = env::current_dir().expect("get current dir");
    let app = dotenv_linter::cli::new(path.as_os_str());
    let matches = app.get_matches_from(vec![
        "dotenv-linter",
        "compare",
        path.join(".env").to_str().expect(".env to str"),
        path.join(".env.compare")
            .to_str()
            .expect(".env.compare to str"),
    ]);

    fs::copy("benches/fixtures/simple.env", path.join(".env")).expect("copy .env file");
    fs::copy("benches/fixtures/compare.env", path.join(".env.compare"))
        .expect("copy .env.compare file");

    c.bench_function("dotenv_linter compare", |b| {
        // Disable output to STDOUT
        #[cfg(not(windows))]
        let _print_gag = Gag::stdout().expect("disable stdout");

        b.iter(|| dotenv_linter::compare(black_box(&matches), black_box(&current_dir)))
    });
}

criterion_group!(benches, compare_benchmark);
criterion_main!(benches);
