use crate::{
    common::LintKind,
    schema::{self, DotEnvSchema},
};
use clap::ArgMatches;
use std::path::PathBuf;
#[derive(Default)]
pub struct CliOptions<'a> {
    pub input: Vec<&'a PathBuf>,
    pub skip: Vec<LintKind>,
    pub exclude: Vec<&'a PathBuf>,
    pub quiet: bool,
    pub recursive: bool,
    pub schema: Option<DotEnvSchema>,
    pub no_backup: bool,
}

pub struct FixOptionsx<'a> {
    pub input: Vec<&'a PathBuf>,
    pub skip: Vec<LintKind>,
    pub exclude: Vec<&'a PathBuf>,
    pub quiet: bool,
    pub recursive: bool,
}

pub struct CompareOptionsx<'a> {
    pub input: Vec<&'a PathBuf>,
    pub quiet: bool,
}

impl<'a> CliOptions<'a> {
    pub fn new_check(args: &'a ArgMatches) -> Self {
        let skip = get_many_from_args::<LintKind>(args, "skip")
            .into_iter()
            .map(|l| l.to_owned())
            .collect();
        let schema = if let Some(schema_path) = args.get_one::<PathBuf>("schema") {
            match schema::DotEnvSchema::load(schema_path) {
                Ok(schema) => Some(schema),
                Err(e) => {
                    println!("Error loading schema: {}", e);
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
            no_backup: false,
        }
    }

    pub fn new_fix(args: &'a ArgMatches) -> Self {
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
            schema: None,
        }
    }

    pub fn new_compare(args: &'a ArgMatches) -> Self {
        Self {
            input: get_many_from_args::<PathBuf>(args, "input"),
            quiet: args.get_flag("quiet"),
            skip: Default::default(),
            exclude: Default::default(),
            recursive: false,
            schema: None,
            no_backup: false,
        }
    }
}

fn get_many_from_args<'a, T: Clone + Send + Sync + 'static>(
    args: &'a ArgMatches,
    name: &'a str,
) -> Vec<&'a T> {
    args.get_many::<T>(name).unwrap_or_default().collect()
}
