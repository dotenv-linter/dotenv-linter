use crate::common::LintKind;
use clap::ArgMatches;
use std::path::PathBuf;

pub struct CheckOptions<'a> {
    pub input: Vec<&'a PathBuf>,
    pub skip: Vec<LintKind>,
    pub exclude: Vec<&'a PathBuf>,
    pub quiet: bool,
    pub recursive: bool,
}

pub struct FixOptions<'a> {
    pub input: Vec<&'a PathBuf>,
    pub skip: Vec<LintKind>,
    pub exclude: Vec<&'a PathBuf>,
    pub quiet: bool,
    pub recursive: bool,
    pub no_backup: bool,
}

pub struct CompareOptions<'a> {
    pub input: Vec<&'a PathBuf>,
    pub quiet: bool,
}

impl<'a> CheckOptions<'a> {
    pub fn new(args: &'a ArgMatches) -> Self {
        let skip = get_many_from_args::<LintKind>(args, "skip")
            .into_iter()
            .map(|l| l.to_owned())
            .collect();

        Self {
            skip,
            input: get_many_from_args::<PathBuf>(args, "input"),
            exclude: get_many_from_args::<PathBuf>(args, "exclude"),
            quiet: args.get_flag("quiet"),
            recursive: args.get_flag("recursive"),
        }
    }
}

impl<'a> FixOptions<'a> {
    pub fn new(args: &'a ArgMatches) -> Self {
        let skip = get_many_from_args::<LintKind>(args, "skip")
            .into_iter()
            .map(|l| l.to_owned())
            .collect();

        Self {
            skip,
            input: get_many_from_args::<PathBuf>(args, "input"),
            exclude: get_many_from_args::<PathBuf>(args, "exclude"),
            quiet: args.get_flag("quiet"),
            recursive: args.get_flag("recursive"),
            no_backup: args.get_flag("no-backup"),
        }
    }
}

impl<'a> CompareOptions<'a> {
    pub fn new(args: &'a ArgMatches) -> Self {
        Self {
            input: get_many_from_args::<PathBuf>(args, "input"),
            quiet: args.get_flag("quiet"),
        }
    }
}

fn get_many_from_args<'a, T: Clone + Send + Sync + 'static>(
    args: &'a ArgMatches,
    name: &'a str,
) -> Vec<&'a T> {
    args.get_many::<T>(name).unwrap_or_default().collect()
}
