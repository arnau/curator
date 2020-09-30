// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use crate::error::Error;
use crate::manifest::Manifest;
use clap::Clap;
use curator_sketch::history::{Record, RecordBuilder};
use curator_sketch::source::{self, Source};
use dialoguer::Editor;
use skim::prelude::*;
use std::fs;
use std::io::Cursor;

#[derive(Debug, Clap)]
pub struct Cmd {
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Clap)]
pub enum Subcommand {
    /// Adds a resource to the history store.
    Add(Add),
}

#[derive(Debug, Clap)]
pub struct Add {
    /// The URL to store.
    url: String,
}

impl Add {
    pub fn run(&mut self, manifest: Manifest) -> Result<(), Error> {
        let source_rdr = fs::File::open(manifest.sources_path())?;
        let sources: Vec<Source> = source::from_reader(source_rdr)?;
        let record = prompt_record(&self.url, &sources)?;

        record.append_into(manifest.history_path())?;

        Ok(())
    }
}

fn prompt_record(url: &str, sources: &[Source]) -> Result<Record, Error> {
    let builder = Record::new(url);
    let template = toml::to_string(&builder)?;
    let record = if let Some(value) = Editor::new().extension(".toml").edit(&template)? {
        let mut entry: RecordBuilder = toml::from_str(&value)?;

        if entry.origin().is_none() {
            let sources = sources
                .iter()
                .map(|src| src.id())
                .collect::<Vec<_>>()
                .join("\n");
            let options = SkimOptionsBuilder::default().height(Some("50%")).build()?;
            let item_reader = SkimItemReader::default();
            let items = item_reader.of_bufread(Cursor::new(sources));
            let selected_items = Skim::run_with(&options, Some(items))
                .map(|out| out.selected_items)
                .unwrap_or_else(|| Vec::new());

            if let Some(item) = selected_items.first() {
                entry.with_origin(item.output());
            }
        }

        entry.build()?
    } else {
        return Err(Error::new("Aborted"));
    };

    Ok(record)
}
