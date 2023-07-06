// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use clap::Parser;

//use curator_sketch;
mod error;
mod history;
mod ideas;
mod manifest;

#[derive(Debug, Parser)]
enum Subcommand {
    /// Manages the history store.
    #[clap(alias = "h")]
    History(history::Cmd),
    /// Manages the idea store.
    Ideas(ideas::Cmd),
}

#[derive(Debug, Parser)]
#[clap(name = "curator", version)]
struct Curator {
    /// Path to the cellar Cellar.toml manifest
    #[clap(
        long,
        short = 'p',
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

    match opts.subcommand {
        Subcommand::History(o) => match o.subcommand {
            history::Subcommand::Add(mut cmd) => cmd.run(manifest),
        },
        Subcommand::Ideas(o) => match o.subcommand {
            ideas::Subcommand::Add(mut cmd) => cmd.run(manifest),
            ideas::Subcommand::List(cmd) => cmd.run(manifest),
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
