// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use crate::error::{Error, TERM_ERR};
use crate::manifest::Manifest;
use clap::Clap;
use dialoguer::{theme::ColorfulTheme, Editor, Input, Select};
use std::process::exit;

#[derive(Debug, Clap)]
pub struct Cmd {
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Clap)]
pub enum Subcommand {
    /// Adds an idea to the idea store.
    Add(Add),
}

#[derive(Debug)]
pub struct Idea {
    content: String,
    reminder: String,
}

#[derive(Debug, Clap)]
pub struct Add;

impl Add {
    pub fn run(&mut self, manifest: Manifest) -> Result<(), Error> {
        use chrono::prelude::*;
        use std::fs::OpenOptions;
        use std::io::Write;

        let path = manifest.ideas_path();
        let mut file = OpenOptions::new().append(true).open(path)?;
        let date = Utc::today().format("%F");

        let theme = ColorfulTheme::default();

        let content = Editor::new()
            .extension(".md")
            .trim_newlines(true)
            .edit("")
            .expect("Couldn't open the editor");

        if content.is_none() {
            exit(0);
        }

        let presets = vec!["tomorrow", "next week", "in an hour", "other"];
        let mut reminder: Option<String> = None;

        if let Some(idx) = Select::with_theme(&theme)
            .items(&presets)
            .with_prompt("Remind me")
            .interact_opt()?
        {
            reminder = match idx {
                0 => Some("P1D".to_string()),
                1 => Some("P7D".to_string()),
                2 => Some("PT1H".to_string()),
                3 => {
                    let val: String = Input::with_theme(&theme)
                        .with_prompt("Reminder (ISO8601)")
                        .allow_empty(true)
                        .interact()?;

                    if val.is_empty() {
                        None
                    } else {
                        Some(val)
                    }
                }
                _ => None,
            }
        }

        // date,content,reminder
        let row = format!(
            "{},\"{}\",{}\n",
            date,
            content.unwrap(),
            reminder.unwrap_or("".to_string())
        );

        file.write(row.as_bytes())?;

        Ok(())
    }
}
