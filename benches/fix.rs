use clap::{ArgMatches, Command};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use std::{env, fs};
use tempfile::tempdir;

#[cfg(not(windows))]
use gag::Gag;

/// Generates clap::ArgMatches for the fix Benchmarks and Copies the needed temporary Files
fn generate_arg_matches(app: &Command, with_backup: bool) -> ArgMatches {
    // Prepare the temporary Files
    let temp = tempdir().expect("create tempdir");
    let path = temp.into_path();
    let simple_fix_path = path.join("simple_fix.env");
    fs::copy("benches/fixtures/simple_fix.env", simple_fix_path.clone()).expect("copy .env file");

    // Prepare the Arguments
    let mut args_vector = vec!["dotenv-linter", "fix"];
    if !with_backup {
        args_vector.push("--no-backup");
    }
    args_vector.push(simple_fix_path.to_str().expect("path to str"));

    // Generate the ArgMatches
    app.clone()
        .get_matches_from(args_vector)
        .subcommand_matches("fix")
        .expect("fix command")
        .to_owned()
}

/// Runs the Fix Benchmark
pub fn fix_benchmark(c: &mut Criterion) {
    let current_dir = env::current_dir().expect("get current dir");
    let app = dotenv_linter::cli::command();
    c.bench_function("dotenv_linter fix", |b| {
        // Disable output to STDOUT
        #[cfg(not(windows))]
        let _print_gag = Gag::stdout().expect("disable stdout");
        // iter_batched, so we can copy the files beforehand
        b.iter_batched(
            || generate_arg_matches(&app, false),
            |arguments| dotenv_linter::fix(black_box(&arguments), black_box(&current_dir)),
            BatchSize::SmallInput,
        )
    });
}
/// Runs the fix Benchmark with Backup
pub fn fix_benchmark_with_backup(c: &mut Criterion) {
    let current_dir = env::current_dir().expect("get current dir");
    let app = dotenv_linter::cli::command();
    c.bench_function("dotenv_linter fix with backup", |b| {
        // Disable output to STDOUT
        #[cfg(not(windows))]
        let _print_gag = Gag::stdout().expect("disable stdout");
        // iter_batched, so we can copy the files beforehand
        b.iter_batched(
            || generate_arg_matches(&app, true),
            |arguments| dotenv_linter::fix(black_box(&arguments), black_box(&current_dir)),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, fix_benchmark, fix_benchmark_with_backup);
criterion_main!(benches);
