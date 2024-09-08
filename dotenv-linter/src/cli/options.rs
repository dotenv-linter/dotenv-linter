use crate::{
    common::LintKind,
    schema::{self, DotEnvSchema},
};
use clap::ArgMatches;
use std::path::PathBuf;

pub struct CheckOptions<'a> {
    pub input: Vec<&'a PathBuf>,
    pub skip: Vec<LintKind>,
    pub exclude: Vec<&'a PathBuf>,
    pub quiet: bool,
    pub recursive: bool,
    pub schema: Option<DotEnvSchema>,
    pub stdin: bool,
}

pub struct FixOptions<'a> {
    pub input: Vec<&'a PathBuf>,
    pub skip: Vec<LintKind>,
    pub exclude: Vec<&'a PathBuf>,
    pub quiet: bool,
    pub recursive: bool,
    pub no_backup: bool,
    pub dry_run: bool,
    pub stdin: bool,
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
        let schema = if let Some(schema_path) = args.get_one::<PathBuf>("schema") {
            match schema::DotEnvSchema::load(schema_path) {
                Ok(schema) => Some(schema),
                Err(err) => {
                    println!("Error loading schema: {}", err);
                    std::process::exit(1);
                }
            }
        } else {
            None
        };

        Self {
            skip,
            input: get_many_from_args::<PathBuf>(args, "input"),
            exclude: get_many_from_args::<PathBuf>(args, "exclude"),
            quiet: args.get_flag("quiet"),
            recursive: args.get_flag("recursive"),
            schema,
            stdin: args.get_flag("stdin"),
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
            dry_run: args.get_flag("dry-run"),
            stdin: args.get_flag("stdin"),
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
