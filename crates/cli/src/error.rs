// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use console::{Style, Term};
use curator_sketch::{history::RecordError, source::SourceError};
use lazy_static::lazy_static;
use std::{error, fmt, io};

lazy_static! {
    pub static ref TERM_ERR: Term = Term::stderr();
    pub static ref TERM_OUT: Term = Term::stdout();
    static ref YELLOW: Style = Style::new().for_stderr().yellow();
    pub static ref GREEN: Style = Style::new().for_stderr().green();
    pub static ref MAGENTA: Style = Style::new().for_stderr().magenta();
}

/// Represents a CLI error.
#[derive(PartialEq, Debug, Clone)]
pub struct Error(String);

impl Error {
    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl error::Error for Error {}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error(format!("{}", err))
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Error {
        Error(format!("{}", err))
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error(format!("{}", err))
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Error {
        Error(format!("{}", err))
    }
}

impl From<RecordError> for Error {
    fn from(err: RecordError) -> Error {
        Error(format!("{}", err))
    }
}

impl From<SourceError> for Error {
    fn from(err: SourceError) -> Error {
        Error(format!("{}", err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
