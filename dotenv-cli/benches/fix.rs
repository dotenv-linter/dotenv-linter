use std::{env, fs, hint::black_box};

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
#[cfg(not(windows))]
use gag::Gag;
use tempfile::tempdir;

pub fn fix_benchmark(c: &mut Criterion) {
    let current_dir = env::current_dir().expect("get current dir");

    // Prepare the temporary files
    let temp = tempdir().expect("create tempdir");
    let path = temp.keep();
    let simple_fix_path = path.join("simple_fix.env");

    c.bench_function("dotenv_linter fix", |b| {
        // Disable output to STDOUT
        #[cfg(not(windows))]
        let _print_gag = Gag::stdout().expect("disable stdout");
        // iter_batched, so we can copy the files beforehand
        b.iter_batched(
            || {
                fs::copy("benches/fixtures/simple_fix.env", simple_fix_path.clone())
                    .expect("copy .env file")
            },
            |_| {
                let opts = dotenv_linter::FixOptions {
                    files: vec![&simple_fix_path],
                    ignore_checks: vec![],
                    exclude: vec![],
                    quiet: false,
                    recursive: false,
                    no_backup: true,
                    dry_run: false,
                };
                dotenv_linter::fix(black_box(&opts), black_box(&current_dir))
            },
            BatchSize::SmallInput,
        )
    });
}

pub fn fix_benchmark_with_backup(c: &mut Criterion) {
    let current_dir = env::current_dir().expect("get current dir");

    // Prepare the temporary files
    let temp = tempdir().expect("create tempdir");
    let path = temp.keep();
    let simple_fix_path = path.join("simple_fix.env");

    c.bench_function("dotenv_linter fix with backup", |b| {
        // Disable output to STDOUT
        #[cfg(not(windows))]
        let _print_gag = Gag::stdout().expect("disable stdout");
        // iter_batched, so we can copy the files beforehand
        b.iter_batched(
            || {
                fs::copy("benches/fixtures/simple_fix.env", simple_fix_path.clone())
                    .expect("copy .env file")
            },
            |_| {
                let opts = dotenv_linter::FixOptions {
                    files: vec![&simple_fix_path],
                    ignore_checks: vec![],
                    exclude: vec![],
                    quiet: false,
                    recursive: false,
                    no_backup: false,
                    dry_run: false,
                };
                dotenv_linter::fix(black_box(&opts), black_box(&current_dir))
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, fix_benchmark, fix_benchmark_with_backup);
criterion_main!(benches);
