// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::OpenOptions;
use std::path::Path;
use std::{fmt, io};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record {
    date: String,
    url: String,
    title: String,
    summary: String,
    #[serde(with = "tags")]
    tags: Vec<String>,
    origin: Option<String>,
}

impl Record {
    pub fn new<S: Into<String>>(url: S) -> RecordBuilder {
        RecordBuilder::new(url)
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn date(&self) -> NaiveDate {
        NaiveDate::parse_from_str(&self.date, "%y-%m-%d").expect("A valid date")
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn origin(&self) -> Option<String> {
        self.origin.clone()
    }

    pub fn write<W: io::Write>(&self, writer: W) -> Result<(), RecordError> {
        let mut wtr = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(writer);
        wtr.serialize(self)?;
        wtr.flush()?;

        Ok(())
    }

    pub fn append_into<P: AsRef<Path>>(&self, path: P) -> Result<(), RecordError> {
        self.write(OpenOptions::new().append(true).open(path)?)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RecordBuilder {
    url: String,
    date: String,
    #[serde(with = "empty_string")]
    title: Option<String>,
    #[serde(with = "empty_string")]
    summary: Option<String>,
    tags: Vec<String>,
    #[serde(with = "empty_string")]
    origin: Option<String>,
}

impl RecordBuilder {
    pub fn new<S: Into<String>>(url: S) -> Self {
        let date = Utc::today();

        RecordBuilder {
            url: url.into(),
            date: date.naive_utc().to_string(),
            title: None,
            summary: None,
            tags: Vec::new(),
            origin: None,
        }
    }

    pub fn with_date(mut self, date: NaiveDate) -> Self {
        self.date = date.to_string();
        self
    }

    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_summary<S: Into<String>>(mut self, summary: S) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn with_tags(mut self, tags: &[&str]) -> Self {
        self.tags = tags.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn with_origin<S: Into<String>>(mut self, origin: S) -> Self {
        self.origin = Some(origin.into());
        self
    }

    pub fn set_origin<S: Into<String>>(&mut self, origin: S) {
        self.origin = Some(origin.into());
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn date(&self) -> NaiveDate {
        NaiveDate::parse_from_str(&self.date, "%y-%m-%d").expect("A valid date")
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }

    pub fn summary(&self) -> Option<String> {
        self.summary.clone()
    }

    pub fn tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn origin(&self) -> Option<String> {
        self.origin.clone()
    }

    /// Builds the actual `Record`.
    ///
    /// Notice that it consumes the builder.
    ///
    /// ## Examples
    ///
    /// ```
    /// use curator_sketch::history::RecordBuilder;
    ///
    /// let b = RecordBuilder::new("https://www.seachess.net")
    ///     .with_title("Seachess")
    ///     .with_summary("A summary")
    ///     .with_tags(&vec!["a", "b", "c"])
    ///     .build();
    ///
    /// assert!(b.is_ok(), "Expected the record to build correctly");
    /// ```
    pub fn build(self) -> Result<Record, RecordError> {
        let record = Record {
            url: self.url,
            date: self.date,
            title: self.title.ok_or(RecordError::MissingTitle)?,
            summary: self.summary.ok_or(RecordError::MissingSummary)?,
            tags: self.tags,
            origin: self.origin,
        };

        Ok(record)
    }
}

mod empty_string {
    use serde::Deserialize;

    pub fn serialize<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *value {
            Some(ref v) => serializer.serialize_some(v),
            None => serializer.serialize_some(""),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let opt = if value.trim().is_empty() {
            None
        } else {
            Some(value.trim().to_string())
        };

        Ok(opt)
    }
}

mod tags {
    use serde::Deserialize;

    pub fn serialize<S>(value: &[String], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&value.join(";"))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let list: Vec<String> = value.split(';').map(|s| s.to_string()).collect();

        Ok(list)
    }
}

#[derive(Debug)]
pub enum RecordError {
    MissingTitle,
    MissingSummary,
    Csv(csv::Error),
    Io(io::Error),
}

impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordError::MissingTitle => write!(f, "'title' is a required field"),
            RecordError::MissingSummary => write!(f, "'summary' is a required field"),
            RecordError::Csv(err) => write!(f, "{}", err),
            RecordError::Io(err) => write!(f, "{}", err),
        }
    }
}

impl Error for RecordError {}

impl From<io::Error> for RecordError {
    fn from(err: io::Error) -> RecordError {
        RecordError::Io(err)
    }
}

impl From<csv::Error> for RecordError {
    fn from(err: csv::Error) -> RecordError {
        RecordError::Csv(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn serialize_builder() -> Result<(), Box<dyn Error>> {
        let builder = RecordBuilder::new("https://www.seachess.net");
        let actual = toml::to_string(&builder)?;
        let expected = "url = \"https://www.seachess.net\"\ndate = \"2020-09-28\"\ntags = []\n";

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn deserialize_builder() -> Result<(), Box<dyn Error>> {
        let builder = RecordBuilder::new("https://www.seachess.net");
        let expected = r#"
            url = "https://www.seachess.net"
            date = "2020-09-28"
            tags = []
            "#;
        let actual: RecordBuilder = toml::from_str(expected)?;

        assert_eq!(actual, builder);

        Ok(())
    }

    #[test]
    fn serde_builder() -> Result<(), Box<dyn Error>> {
        let builder = RecordBuilder::new("https://www.seachess.net");
        let actual: RecordBuilder = toml::from_str(&toml::to_string(&builder)?)?;

        assert_eq!(actual, builder);

        Ok(())
    }

    #[test]
    fn builder_chain() {
        let builder = RecordBuilder::new("https://www.seachess.net")
            .with_date(NaiveDate::from_ymd(2020, 9, 20))
            .with_title("Seachess");

        assert_eq!(builder.title(), Some("Seachess".to_string()));
    }
}
