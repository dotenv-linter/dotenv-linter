//! Benchmark for the `compare` subcommand using Criterion.
//!
//! How to run:
//! - All benches: `cargo bench`
//! - Only this bench: `cargo bench --bench compare`
//! - With verbose output: `RUST_LOG=info cargo bench --bench compare`
//!
//! Interpreting results:
//! - Look at `time:   [..]` (mean/median) to track latency per iteration.
//! - Compare over commits to catch regressions or improvements.
//! - Use Criterion’s regression detection to alert on statistically significant changes.

use std::{env, fs, hint::black_box};

use criterion::{criterion_group, criterion_main, Criterion};
#[cfg(not(windows))]
use gag::Gag;
use tempfile::tempdir;

pub fn compare_benchmark(c: &mut Criterion) {
    let temp = tempdir().expect("create tempdir");
    let path = temp.keep();

    let current_dir = env::current_dir().expect("get current dir");
    let app = dotenv_linter::cli::command();
    let args = app.get_matches_from(vec![
        "dotenv-linter",
        "compare",
        path.join(".env").to_str().expect(".env to str"),
        path.join(".env.compare")
            .to_str()
            .expect(".env.compare to str"),
    ]);
    let opts = dotenv_linter::cli::options::CompareOptions::new(&args);

    // Use small, realistic fixtures to avoid I/O dominating the benchmark
    fs::copy("benches/fixtures/simple.env", path.join(".env")).expect("copy .env file");
    fs::copy("benches/fixtures/compare.env", path.join(".env.compare"))
        .expect("copy .env.compare file");

    c.bench_function("dotenv_linter compare", |b| {
        // Disable output to STDOUT to reduce noise during benchmarking
        #[cfg(not(windows))]
        let _print_gag = Gag::stdout().expect("disable stdout");

        b.iter(|| dotenv_linter::compare(black_box(&opts), black_box(&current_dir)))
    });
}

criterion_group!(benches, compare_benchmark);
criterion_main!(benches);
