// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use crate::error::Error;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    activity: Activity,
}

impl Manifest {
    pub fn new(path: &str) -> Result<Manifest, Error> {
        let mut content = String::new();
        File::open(&path).and_then(|mut f| f.read_to_string(&mut content))?;

        toml::from_str(&content).map_err(|err| Error::new(&err.to_string()))
    }

    pub fn history_path(&self) -> &str {
        &self.activity.history
    }

    pub fn ideas_path(&self) -> &str {
        &self.activity.ideas
    }
}

#[derive(Debug, Deserialize)]
pub struct Activity {
    history: String,
    events: String,
    sources: String,
    ideas: String,
}
