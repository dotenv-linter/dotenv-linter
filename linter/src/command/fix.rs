use crate::common::FixOutput;
use crate::{checks, fixes, Linter};

pub fn fix(opts: Linter) -> crate::Result<usize> {
    let output = FixOutput::new(opts.quiet);

    if opts.files.is_empty() {
        output.print_nothing_to_fix();
        return Ok(0);
    }

    let output = output.files_count(opts.files.count());

    let mut warnings_count = 0;
    for (index, (fe, mut lines)) in opts.files.into_iter().enumerate() {
        output.print_processing_info(&fe);
        let result = checks::run(&lines, opts.skip_checks);
        if result.is_empty() {
            continue;
        }
        let fixes_done = fixes::run(&result, &mut lines, opts.skip_checks);
        if fixes_done != result.len() {
            output.print_not_all_warnings_fixed();
        }

        // TODO: Backup file by implementing method `backup` for FileEntry
        if fixes_done > 0 {
            // create backup copy unless user specifies not to
            if opts.backup {
                // let backup_file = fs_utils::backup_file(&fe)?;
                // output.print_backup(&backup_file);
            }

            // write corrected file
            // fs_utils::write_file(&fe.path, lines)?;
        }

        output.print_warnings(&fe, &result, index);
        warnings_count += result.len();
    }

    output.print_total(warnings_count);
    Ok(warnings_count)
}
