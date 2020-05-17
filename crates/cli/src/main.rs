// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use clap::{AppSettings, Clap};

//use curator_sketch;
mod error;
mod history;
mod manifest;

#[derive(Debug, Clap)]
enum Subcommand {
    /// Manages the reading history.
    History(history::Cmd),
}

#[derive(Debug, Clap)]
#[clap(name = "curator", version, global_setting(AppSettings::ColoredHelp))]
struct Curator {
    /// Path to the cellar Cellar.toml manifest
    #[clap(
        long,
        short = "p",
        value_name = "path",
        default_value = "./Cellar.toml"
    )]
    manifest_path: String,

    /// Verbose mode
    #[clap(short)]
    verbose: bool,

    #[clap(subcommand)]
    subcommand: Subcommand,
}

fn main() -> Result<(), error::Error> {
    let opts: Curator = Curator::parse();
    let manifest = manifest::Manifest::new(&opts.manifest_path)?;
    println!("{:?}", manifest);

    match opts.subcommand {
        Subcommand::History(h) => match h.subcommand {
            history::Subcommand::Add(mut cmd) => cmd.run(manifest),
        },
    }

    // let code = if let Some(error) = err {
    //     error.print_err().unwrap();
    //     1
    // } else {
    //     0
    // };

    // TERM_ERR.flush().unwrap();
    // TERM_OUT.flush().unwrap();

    // exit(code)
}
