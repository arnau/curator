// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use crate::error::Error;
use crate::manifest::Manifest;
use chrono::prelude::*;
use clap::Clap;
use console::{Style, Term};
use dialoguer::{theme::ColorfulTheme, Editor, Input, Select};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
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
    /// Lists all ideas in the store.
    List(List),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Idea {
    date: String,
    content: String,
    reminder: Option<String>,
}

#[derive(Debug, Clap)]
pub struct Add;

impl Add {
    pub fn run(&mut self, manifest: Manifest) -> Result<(), Error> {
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

#[derive(Debug, Clap)]
pub struct List;

impl List {
    pub fn run(&self, manifest: Manifest) -> Result<(), Error> {
        let path = manifest.ideas_path();
        let f = File::open(path)?;
        let mut rdr = csv::Reader::from_reader(f);
        // let date = Utc::today().format("%F");

        // use std::time::Duration;
        let term = Term::stdout();
        let (_height, width) = term.size();

        let hi_row = Style::new().on_black().on_bright();
        let head = Style::new().on_black().white();

        {
            let headers = rdr.headers()?;
            let header = format!(
                "###  {:10}  {:10}  {}",
                headers.get(0).unwrap(),
                headers.get(2).unwrap(),
                headers.get(1).unwrap(),
            );
            println!("{:80}", head.apply_to(header));
        }

        for (idx, result) in rdr.deserialize().enumerate() {
            let record: Idea = result?;
            let date = &record.date;
            let summary = &record.content.lines().next().unwrap();
            let reminder = record.reminder.clone().unwrap_or("".to_string());
            let row = format!("{:3}  {:10}  {:10}  {}", idx, &date, &reminder, &summary);
            let padding = width as usize - row.len();
            let row_padded = format!("{}{}", row, " ".repeat(padding));

            if idx % 2 == 0 {
                println!("{}", hi_row.apply_to(row_padded));
            } else {
                println!("{}", row_padded);
            }
        }
        Ok(())
    }

    fn run_csv(&self, manifest: Manifest) -> Result<(), Error> {
        let path = manifest.ideas_path();
        let f = File::open(path)?;
        let mut rdr = csv::Reader::from_reader(f);
        let mut wtr = csv::Writer::from_writer(Term::stdout());

        for result in rdr.deserialize() {
            let mut record: Idea = result?;

            if self.summary {
                record.content = record.content.lines().next().unwrap().to_string();
            }

            wtr.serialize(record)?;
        }
        wtr.flush()?;

        Ok(())
    }
}
