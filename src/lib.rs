use crate::{common::*, quote_type::QuoteType};
use clap::parser::ValuesRef;
use std::{
    collections::{BTreeMap, HashSet},
    path::{Path, PathBuf},
    str::FromStr,
};

pub use checks::available_check_names;

mod checks;
mod common;
mod fixes;
mod fs_utils;

pub mod cli;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn check(args: &clap::ArgMatches, current_dir: &Path) -> Result<usize> {
    let lines_map = get_lines(
        args,
        current_dir,
        args.get_flag("recursive"),
        args.get_many("exclude"),
    );
    let output = CheckOutput::new(args.get_flag("quiet"), lines_map.len());

    if lines_map.is_empty() {
        output.print_nothing_to_check();

        #[cfg(feature = "update-informer")]
        print_new_version_if_available(args);

        return Ok(0);
    }

    let skip_checks: Vec<LintKind> = args
        .get_many::<String>("skip")
        .unwrap_or_default()
        .filter_map(|check| LintKind::from_str(check).ok())
        .collect();

    let warnings_count =
        lines_map
            .into_iter()
            .enumerate()
            .fold(0, |acc, (index, (fe, strings))| {
                output.print_processing_info(&fe);

                let lines = get_line_entries(strings);
                let result = checks::run(&lines, &skip_checks);

                output.print_warnings(&fe, &result, index);
                acc + result.len()
            });

    output.print_total(warnings_count);

    #[cfg(feature = "update-informer")]
    print_new_version_if_available(args);

    Ok(warnings_count)
}

pub fn fix(args: &clap::ArgMatches, current_dir: &Path) -> Result<()> {
    let mut warnings_count = 0;
    let lines_map = get_lines(
        args,
        current_dir,
        args.get_flag("recursive"),
        args.get_many("exclude"),
    );
    let output = FixOutput::new(args.get_flag("quiet"), lines_map.len());

    // Nothing to fix
    if lines_map.is_empty() {
        output.print_nothing_to_fix();
        return Ok(());
    }

    let skip_checks: Vec<LintKind> = args
        .get_many::<String>("skip")
        .unwrap_or_default()
        .filter_map(|check| LintKind::from_str(check).ok())
        .collect();

    for (index, (fe, strings)) in lines_map.into_iter().enumerate() {
        output.print_processing_info(&fe);

        let mut lines = get_line_entries(strings);
        let result = checks::run(&lines, &skip_checks);
        if result.is_empty() {
            continue;
        }
        let fixes_done = fixes::run(&result, &mut lines, &skip_checks);
        if fixes_done != result.len() {
            output.print_not_all_warnings_fixed();
        }
        if fixes_done > 0 {
            let should_backup = !args.get_flag("no-backup");
            // create backup copy unless user specifies not to
            if should_backup {
                let backup_file = fs_utils::backup_file(&fe)?;
                output.print_backup(&backup_file);
            }

            // write corrected file
            fs_utils::write_file(&fe.path, lines)?;
        }

        output.print_warnings(&fe, &result, index);
        warnings_count += result.len();
    }

    output.print_total(warnings_count);
    Ok(())
}

// Compares if different environment files contains the same variables and returns warnings if not
pub fn compare(args: &clap::ArgMatches, current_dir: &Path) -> Result<Vec<CompareWarning>> {
    let mut all_keys: HashSet<String> = HashSet::new();
    let lines_map = get_lines(args, current_dir, false, None);
    let output = CompareOutput::new(args.get_flag("quiet"));

    let mut warnings: Vec<CompareWarning> = Vec::new();
    let mut files_to_compare: Vec<CompareFileType> = Vec::new();

    // Nothing to check
    if lines_map.is_empty() {
        output.print_nothing_to_compare();
        return Ok(warnings);
    }

    // Create CompareFileType structures for each file
    for (_, (fe, strings)) in lines_map.into_iter().enumerate() {
        output.print_processing_info(&fe);
        let lines = get_line_entries(strings);
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
    Ok(warnings)
}

fn get_lines(
    args: &clap::ArgMatches,
    current_dir: &Path,
    is_recursive: bool,
    exclude: Option<ValuesRef<String>>,
) -> BTreeMap<FileEntry, Vec<String>> {
    let file_paths: Vec<PathBuf> = get_needed_file_paths(args, current_dir, is_recursive, exclude);

    file_paths
        .iter()
        .filter_map(|path: &PathBuf| -> Option<(FileEntry, Vec<String>)> {
            fs_utils::get_relative_path(path, current_dir).and_then(FileEntry::from)
        })
        .collect()
}

/// Getting a list of all files for checking/fixing without custom exclusion files
fn get_needed_file_paths(
    args: &clap::ArgMatches,
    current_dir: &Path,
    is_recursive: bool,
    exclude: Option<ValuesRef<String>>,
) -> Vec<PathBuf> {
    let mut file_paths: Vec<PathBuf> = Vec::new();
    let mut excluded_paths: Vec<PathBuf> = Vec::new();

    if let Some(excluded) = exclude {
        excluded_paths = excluded
            .filter_map(|f| fs_utils::canonicalize(f).ok())
            .collect();
    }

    match args.get_many::<String>("input") {
        Some(inputs) => {
            let input_paths = inputs
                .filter_map(|s| fs_utils::canonicalize(s).ok())
                .collect();

            file_paths.extend(get_file_paths(input_paths, &excluded_paths, is_recursive));
        }
        None => {
            file_paths.extend(get_file_paths(
                vec![current_dir.to_path_buf()],
                &excluded_paths,
                is_recursive,
            ));
        }
    }

    if let Some(inputs) = args.get_many("input") {
        let input_paths = inputs
            .filter_map(|s: &String| fs_utils::canonicalize(s).ok())
            .collect();

        file_paths.extend(get_file_paths(input_paths, &excluded_paths, is_recursive));
    }

    file_paths
}

fn get_file_paths(
    dir_entries: Vec<PathBuf>,
    excludes: &[PathBuf],
    is_recursive: bool,
) -> Vec<PathBuf> {
    let nested_paths: Vec<PathBuf> = dir_entries
        .iter()
        .filter(|entry| entry.is_dir())
        .filter(|entry| !excludes.contains(entry))
        .filter_map(|dir| dir.read_dir().ok())
        .map(|read_dir| {
            read_dir
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|path| {
                    FileEntry::is_env_file(path)
                        || (is_recursive && path.is_dir() && path.read_link().is_err())
                })
                .collect()
        })
        .flat_map(|dir_entries| get_file_paths(dir_entries, excludes, is_recursive))
        .collect();

    let mut file_paths: Vec<PathBuf> = dir_entries
        .into_iter()
        .filter(|entry| entry.is_file())
        .filter(|entry| !excludes.contains(entry))
        .collect();

    file_paths.extend(nested_paths);
    file_paths.sort();
    file_paths.dedup();
    file_paths
}

fn get_line_entries(lines: Vec<String>) -> Vec<LineEntry> {
    let length = lines.len();

    let mut lines: Vec<LineEntry> = lines
        .into_iter()
        .enumerate()
        .map(|(index, line)| LineEntry::new(index + 1, line, length == (index + 1)))
        .collect();

    reduce_multiline_entries(&mut lines);
    lines
}

fn reduce_multiline_entries(lines: &mut Vec<LineEntry>) {
    let length = lines.len();
    let multiline_ranges = find_multiline_ranges(lines);

    // Replace multiline value to one line-entry for checking
    let mut offset = 1; // index offset to account deleted lines (for access by index)
    for (start, end) in multiline_ranges {
        let result = lines
            .drain(start - offset..end - offset + 1) // TODO: consider `drain_filter` (after stabilization in rust std)
            .map(|entry| entry.raw_string)
            .reduce(|result, line| result + "\n" + &line); // TODO: `intersperse` (after stabilization in rust std)

        if let Some(value) = result {
            lines.insert(start - offset, LineEntry::new(start, value, length == end));
        }

        offset += end - start;
    }
}

fn find_multiline_ranges(lines: &[LineEntry]) -> Vec<(usize, usize)> {
    let mut multiline_ranges: Vec<(usize, usize)> = Vec::new();
    let mut start_number: Option<usize> = None;
    let mut quote_char: Option<char> = None;

    // here we find ranges of multi-line values
    lines.iter().for_each(|entry| {
        if let Some(start) = start_number {
            if let Some(quote_char) = quote_char {
                if let Some(idx) = entry.raw_string.find(quote_char) {
                    if !is_escaped(&entry.raw_string[..idx]) {
                        multiline_ranges.push((start, entry.number));
                        start_number = None;
                    }
                }
            }
        } else if let Some(trimmed_value) = entry.get_value().map(|val| val.trim()) {
            if let Some(quote_type) = is_multiline_start(trimmed_value) {
                quote_char = Some(quote_type.char());
                start_number = Some(entry.number);
            }
        }
    });

    multiline_ranges
}

/// Returns the `QuoteType` for a `&str` starting with a quote-char
fn is_multiline_start(val: &str) -> Option<QuoteType> {
    [QuoteType::Single, QuoteType::Double]
        .into_iter()
        .find(|quote_type| quote_type.is_quoted_value(val))
}

/// Prints information about the new version to `STDOUT` if a new version is available
#[cfg(feature = "update-informer")]
fn print_new_version_if_available(args: &clap::ArgMatches) {
    use colored::*;
    use update_informer::{registry, Check};

    if args.get_flag("not-check-updates") || args.get_flag("quiet") {
        return;
    }

    let pkg_name = env!("CARGO_PKG_NAME");

    #[cfg(not(feature = "stub_check_version"))]
    let current_version = env!("CARGO_PKG_VERSION");
    #[cfg(feature = "stub_check_version")]
    let current_version = "3.0.0";

    #[cfg(not(feature = "stub_check_version"))]
    let informer = update_informer::new(registry::Crates, pkg_name, current_version);
    #[cfg(feature = "stub_check_version")]
    let informer = update_informer::fake(registry::Crates, pkg_name, current_version, "3.1.1");

    if let Ok(Some(version)) = informer.check_version() {
        let msg = format!(
            "A new release of {pkg_name} is available: v{current_version} -> {new_version}",
            pkg_name = pkg_name.italic().cyan(),
            current_version = current_version,
            new_version = version.to_string().green()
        );

        let release_url = format!(
            "https://github.com/{pkg_name}/{pkg_name}/releases/tag/{version}",
            pkg_name = pkg_name,
            version = version
        )
        .yellow();

        println!("\n{msg}\n{url}", msg = msg, url = release_url);
    }
}
