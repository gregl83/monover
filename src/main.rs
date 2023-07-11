//! Monorepository Versioning Agent.
//!
//! For help:
//! ```bash
//! cargo run -- -h
//! ```

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::unreachable;
use clap::{
    crate_name,
    crate_description,
    crate_version,
    builder::TypedValueParser,
    error::ErrorKind,
    error::ContextKind,
    error::ContextValue,
    Command,
    Arg
};
use std::path::PathBuf;
use rayon::prelude::*;
use walkdir::{WalkDir, DirEntry};

mod package_managers;

#[derive(Debug)]
enum ReconciliationError {
    IoError,
    ParseError,
}

impl fmt::Display for ReconciliationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError => write!(f, "I/O error"),
            Self::ParseError => write!(f, "Parse error"),
        }
    }
}

impl Error for ReconciliationError {}

/// Abstraction of package with paths to key files for versioning.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Package {
    version_file_path: Option<PathBuf>,
    change_file_path: Option<PathBuf>,
    package_file_paths: Vec<PathBuf>,
}

impl Package {
    fn new() -> Self {
        Self {
            version_file_path: None,
            change_file_path: None,
            package_file_paths: Vec::new(),
        }
    }
}

/// Repository of packages to version.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Repository<'a> {
    root: PathBuf,
    managers: &'a package_managers::Collection,
    packages: HashMap<String, Package>,
}

impl<'a> Repository<'a> {
    fn new(root: PathBuf, managers: &package_managers::Collection, paths: Vec<PathBuf>) -> Self {
        Self {
            root,
            managers,
            packages: Self::parse_paths(paths),
        }
    }

    fn parse_paths(paths: Vec<PathBuf>) -> HashMap<String, Package> {
        let mut packages = HashMap::new();
        for path in paths {
            let package_path = path.parent().unwrap();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let package_version_file_path = package_path.join("VERSION");
            if path == package_version_file_path {
                continue;
            }
            let package = packages.entry(
                package_path.to_str().unwrap().to_string()
            ).or_insert_with(|| Package::new());
            match file_name {
                "VERSION" => {
                    package.version_file_path = Some(path);
                }
                "CHANGE" => {
                    package.change_file_path = Some(path);
                }
                _ => {
                    package.package_file_paths.push(path);
                }
            }
        }
        packages
    }

    fn reconcile(self) -> Result<(), ReconciliationError> {
        for package in self.packages.values() {
            if package.change_file_path.is_none() {
                continue;
            }
            // todo - if there is a version file, decide if you use a package file or not
            if package.version_file_path.is_none() {
                // todo - process package file
                continue;
            }
            // todo -
            println!("{:?}", package);
        }
        Ok(())
    }
}

fn is_reserved_file(file_name: &str) -> bool {
    file_name == "CHANGE" || file_name == "VERSION"
}

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

fn get_paths(
    root: &PathBuf,
    managers: &package_managers::Collection,
    ignore_hidden: bool
) -> Vec<PathBuf> {
    // todo - fix thread reference in objects
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
                        let file_name = entry.file_name().to_str().unwrap();
                        if is_reserved_file(file_name) || managers.has_file_match(file_name) {
                            acc.push(entry.into_path());
                        }
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

    // commands:
    //   - reconcile - updates ALL versions while removing CHANGE files
    //   - publish - version control based commit, tag and push

    match matches.subcommand() {
        Some(("reconcile", sub_matches)) => {
            let repository_root = sub_matches.get_one::<PathBuf>(
                "repository"
            ).expect("required");

            let managers = package_managers::Collection::new();

            let paths = get_paths(
                repository_root,
                &managers,
                true
            );

            let repository = Repository::new(
                repository_root.clone(),
                &managers,
                paths
            );

            println!("{:?}", repository);

            let result = repository.reconcile();

            println!("{:?}", result);

            // validation - version file should exist

            // for each package in repository:
            //   - find CHANGE and VERSION files
            //   - find package files

            // todo - match CHANGE and VERSION files to package files

            // todo - error collection for non-matches

            // todo - process matches
            // todo -   validate target version
            // todo -   update package version
            // todo -   remove CHANGE files

            // todo - confirm complete and report
        }
        _ => unreachable!(),
    }
}
