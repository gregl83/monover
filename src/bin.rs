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
    Command
};

fn cli() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(
            Command::new("reconcile")
                .about("Reconcile CHANGE files to versions in repository")
                .arg(arg!(<REPOSITORY> "Path to repository"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("reconcile", sub_matches)) => {
            println!(
                "Cloning {}",
                sub_matches.get_one::<String>("REPOSITORY").expect("required")
            );
        }
        _ => unreachable!(),
    }
}
