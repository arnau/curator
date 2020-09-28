// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use crate::error::Error;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use toml;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    activity: Activity,
}

impl Manifest {
    pub fn new(path: &str) -> Result<Manifest, Error> {
        let mut content = String::new();
        File::open(path).and_then(|mut f| f.read_to_string(&mut content))?;

        let mut manifest: Manifest =
            toml::from_str(&content).map_err(|err| Error::new(&err.to_string()))?;

        if let Some(base) = Path::new(path).canonicalize()?.parent() {
            manifest.activity.history = base.join(manifest.activity.history);
            manifest.activity.events = base.join(manifest.activity.events);
            manifest.activity.sources = base.join(manifest.activity.sources);
            manifest.activity.ideas = base.join(manifest.activity.ideas);
        }

        Ok(manifest)
    }

    pub fn history_path(&self) -> &Path {
        &self.activity.history
    }

    pub fn events_path(&self) -> &Path {
        &self.activity.events
    }

    pub fn sources_path(&self) -> &Path {
        &self.activity.sources
    }

    pub fn ideas_path(&self) -> &Path {
        &self.activity.ideas
    }
}

#[derive(Debug, Deserialize)]
pub struct Activity {
    history: PathBuf,
    events: PathBuf,
    sources: PathBuf,
    ideas: PathBuf,
}
