// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use crate::error::{Error, TERM_ERR};
use crate::manifest::Manifest;
use clap::Clap;
use dialoguer::{theme::ColorfulTheme, Input};

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

    /// The title of the resource.
    #[clap(long)]
    title: Option<String>,

    /// The short explanation of what is the resource about.
    #[clap(long)]
    summary: Option<String>,

    /// The list of tags to classify the resource.
    #[clap(long, short = "t")]
    tags: Vec<String>,

    /// The origin source where the resource was found.
    #[clap(long)]
    origin: Option<String>,
}

impl Add {
    pub fn run(&mut self, manifest: Manifest) -> Result<(), Error> {
        use chrono::prelude::*;
        use std::fs::OpenOptions;
        use std::io::Write;

        let path = manifest.history_path();
        let mut file = OpenOptions::new().append(true).open(path)?;
        let date = Utc::today().format("%F");

        let theme = ColorfulTheme::default();

        if self.title.is_none() {
            self.title = Some(
                Input::with_theme(&theme)
                    .with_prompt("Title")
                    .interact_on(&TERM_ERR)?,
            );
        }

        if self.summary.is_none() {
            self.summary = Some(
                Input::with_theme(&theme)
                    .with_prompt("Summary")
                    .interact_on(&TERM_ERR)?,
            );
        }

        if self.tags.is_empty() {
            let tags: String = Input::with_theme(&theme)
                .with_prompt("Tags")
                .interact_on(&TERM_ERR)?;

            self.tags = tags
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
        }

        if self.origin.is_none() {
            let origin: String = Input::with_theme(&theme)
                .with_prompt("Origin")
                .allow_empty(true)
                .interact_on(&TERM_ERR)?;

            self.origin = if origin.is_empty() {
                None
            } else {
                Some(origin)
            };
        }

        // let types = vec!["library", "binary"];

        // let template = Select::with_theme(&theme)
        //     .items(&types)
        //     .default(1)
        //     .with_prompt("Type of the crate")
        //     .interact_on(&TERM_ERR)?;

        let tags = self.tags.join(";");
        let title = self.title.clone().unwrap();
        let summary = self.summary.clone().unwrap();
        let origin = self.origin.clone().unwrap_or("".to_string());
        let row = format!(
            "{},{},\"{}\",\"{}\",\"{}\",{}\n",
            date, self.url, title, summary, tags, origin,
        );

        file.write(row.as_bytes())?;

        Ok(())
    }
}
