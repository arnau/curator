// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::io::{self, Read};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Source {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    url: String,
    comment: String,
}

impl Source {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn type_(&self) -> &str {
        &self.type_
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

pub fn from_reader<R: Read>(reader: R) -> Result<Vec<Source>, SourceError> {
    let mut sources = Vec::new();
    let mut rdr = csv::Reader::from_reader(reader);

    for result in rdr.deserialize() {
        let record: Source = result?;
        sources.push(record);
    }

    Ok(sources)
}

/// Represents a CLI error.
#[derive(PartialEq, Debug, Clone)]
pub struct SourceError(String);

impl SourceError {
    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl Error for SourceError {}

impl From<io::Error> for SourceError {
    fn from(err: io::Error) -> SourceError {
        SourceError(format!("{}", err))
    }
}

impl From<csv::Error> for SourceError {
    fn from(err: csv::Error) -> SourceError {
        SourceError(format!("{}", err))
    }
}

impl From<toml::de::Error> for SourceError {
    fn from(err: toml::de::Error) -> SourceError {
        SourceError(format!("{}", err))
    }
}

impl From<toml::ser::Error> for SourceError {
    fn from(err: toml::ser::Error) -> SourceError {
        SourceError(format!("{}", err))
    }
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
