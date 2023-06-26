//! Monorepository Versioning Agent.
//!
//! For help:
//! ```bash
//! cargo run -- -h
//! ```

use std::unreachable;
use clap::{
    crate_name,
    crate_description,
    crate_version,
    arg,
    builder::TypedValueParser,
    error::ErrorKind,
    error::ContextKind,
    error::ContextValue,
    Command,
    Arg,
    ArgAction
};
use std::path::PathBuf;
pub use arrayvec::ArrayString;
use rayon::prelude::*;
use walkdir::{WalkDir, DirEntry};

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub struct PathBufferValueParser {}

impl TypedValueParser for PathBufferValueParser {
    type Value = PathBuf;

    fn parse_ref(
        &self,
        cmd: &Command,
        arg: Option<&Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let path = PathBuf::from(value);
        if !path.exists() {
            let mut err = clap::Error::new(ErrorKind::InvalidValue).with_cmd(cmd);
            err.insert(
                ContextKind::InvalidArg,
                ContextValue::String(arg.unwrap().to_string())
            );
            err.insert(
                ContextKind::InvalidValue,
                ContextValue::String(value.to_string_lossy().into_owned())
            );
            err.insert(
                ContextKind::ValidValue,
                ContextValue::Strings(vec![
                    String::from("valid file or directory path")
                ])
            );
            return Err(err);
        }
        Ok(path)
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s != "." && s.starts_with("."))
         .unwrap_or(false)
}

fn filter(ignore_hidden: bool) -> impl FnMut(&DirEntry) -> bool {
    if ignore_hidden {
        |entry: &DirEntry| -> bool { !is_hidden(entry) }
    } else {
        |_: &DirEntry| -> bool { true }
    }
}

fn get_paths(root: &PathBuf, ignore_hidden: bool) -> Vec<PathBuf> {
    WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(filter(ignore_hidden))
        .par_bridge() // Convert the iterator to a parallel iterator
        .fold(
            || Vec::new(),
            |mut acc, entry| {
                match entry {
                    Ok(entry) => {
                        acc.push(entry.into_path());
                    },
                    _ => {}
                }
                acc
            },
        )
        .reduce(
            || Vec::new(),
            |mut paths_a, paths_b| {
                paths_a.extend(paths_b.into_iter());
                paths_a
            },
        )
}

fn cli() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(
            Command::new("reconcile")
                .about("Reconcile repository versions.")
                .arg(
                    Arg::new("repository")
                    .help("Repository path to reconcile.")
                    .value_parser(PathBufferValueParser{})
                    .default_value(".")
                )
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("reconcile", sub_matches)) => {
            let repository = sub_matches.get_one::<PathBuf>("repository").expect("required");
            let mut paths = get_paths(repository, true);
            paths.sort_unstable();

            // todo - search for CHANGE and VERSION files

            // todo - match CHANGE and VERSION files to package files

            // todo - error collection for non-matches

            // todo - process matches
            // todo -   validate target version
            // todo -   update package version
            // todo -   remove CHANGE files

            // todo - confirm complete and report

            println!(
                "paths {:?}",
                paths
            );
        }
        _ => unreachable!(),
    }
}
