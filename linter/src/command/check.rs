use crate::common::CheckOutput;
use crate::{checks, Linter};

pub fn check(opts: Linter) -> crate::Result<usize> {
    let output = CheckOutput::new(opts.quiet);

    if opts.files.is_empty() {
        output.print_nothing_to_check();
        return Ok(0);
    }

    let output = output.files_count(opts.files.count());

    let warnings_count = opts
        .files
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, (fe, lines))| {
            output.print_processing_info(&fe);
            let result = checks::run(&lines, opts.skip_checks);

            output.print_warnings(&fe, &result, index);
            acc + result.len()
        });

    output.print_total(warnings_count);
    Ok(warnings_count)
}
