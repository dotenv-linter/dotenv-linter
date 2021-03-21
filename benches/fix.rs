use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use std::{env, fs};
use tempfile::tempdir;

#[cfg(not(windows))]
use gag::Gag;

pub fn fix_benchmark(c: &mut Criterion) {
    let current_dir = env::current_dir().expect("get current dir");
    let app = dotenv_linter::cli::new(current_dir.as_os_str());
    c.bench_function("dotenv_linter fix", |b| {
        // Disable output to STDOUT
        #[cfg(not(windows))]
        let _print_gag = Gag::stdout().expect("disable stdout");
        // iter_batched, so we can copy the files beforehand
        b.iter_batched(
            || {
                let temp = tempdir().expect("create tempdir");
                let path = temp.into_path();
                let simple_fix_path = path.join("simple_fix.env");

                let matches = app.clone().get_matches_from(vec![
                    "dotenv-linter",
                    // Flag doesn't work like this
                    // "no-backup",
                    simple_fix_path.to_str().expect("path to str"),
                ]);

                fs::copy("benches/fixtures/simple_fix.env", simple_fix_path)
                    .expect("copy .env file");
                matches
            },
            |arguments| dotenv_linter::fix(black_box(&arguments), black_box(&current_dir)),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, fix_benchmark);
criterion_main!(benches);
