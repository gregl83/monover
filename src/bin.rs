//! Monorepository Versioning Agent.
//!
//! For help:
//! ```bash
//! cargo run -- -h
//! ```

use clap::{App, Arg};

fn main() {
    // todo - add error handling with messaging

    let _matches = App::new("monover")
        .version("0.1.0")
        .about("Blazing fast intelligent monorepo continuous integration versioning.")
        // .arg(Arg::with_name("src")
        //     .help("Source to hash (filesystem path)")
        //     .default_value(".")
        //     .index(1))
        // .arg(Arg::with_name("ignore-hidden")
        //     .help("Ignore files or directories starting with dot or full stop")
        //     .long("ignore-hidden")
        //     .short("i")
        // )
        .get_matches();

    // let source = matches.value_of("src").unwrap();
    // let ignore_hidden = matches.is_present("ignore-hidden");
    //let hash = hash_source(source, ignore_hidden);
    //println!("{}", hash);
}
