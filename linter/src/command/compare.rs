use crate::common::{CompareFileType, CompareOutput, CompareWarning};
use crate::Linter;
use std::collections::HashSet;

// Compares if different environment files contains the same variables and returns warnings if not
pub fn compare(opts: Linter) -> crate::Result<usize> {
    let output = CompareOutput::new(opts.quiet);
    let mut warnings: Vec<CompareWarning> = Vec::new();

    if opts.files.is_empty() {
        output.print_nothing_to_compare();
        return Ok(0);
    }

    // Create CompareFileType structures for each file
    let mut all_keys: HashSet<String> = HashSet::new();
    let mut files_to_compare: Vec<CompareFileType> = Vec::new();
    for (_, (fe, lines)) in opts.files.into_iter().enumerate() {
        output.print_processing_info(&fe);
        let mut keys: Vec<String> = Vec::new();

        for line in lines {
            if let Some(key) = line.get_key() {
                all_keys.insert(key.to_string());
                keys.push(key.to_string());
            }
        }

        let file_to_compare: CompareFileType = CompareFileType {
            path: fe.path,
            keys,
            missing: Vec::new(),
        };

        files_to_compare.push(file_to_compare);
    }

    // Create warnings if any file misses any key
    for file in files_to_compare {
        let missing_keys: Vec<_> = all_keys
            .iter()
            .filter(|key| !file.keys.contains(key))
            .map(|key| key.to_owned())
            .collect();

        if !missing_keys.is_empty() {
            let warning = CompareWarning {
                path: file.path,
                missing_keys,
            };

            warnings.push(warning)
        }
    }

    output.print_warnings(&warnings);
    Ok(warnings.len())
}
