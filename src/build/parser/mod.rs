pub mod markdown;

use chrono::{DateTime, Utc};
use thiserror::Error;

pub trait Parser {
    fn new() -> Self;
    fn parse(&self, text: &str) -> Result<ParsedData, ParserError>;
}

// TODO: handle more values
// TODO: only use Post struct?
#[derive(Debug)]
pub struct ParsedData {
    pub title: String,
    pub description: String,
    pub published_at: DateTime<Utc>,
    pub content: String,
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Empty value for parsed file")]
    EmptyValueError,

    #[error("Could not parse markdown field: {0}")]
    MarkdownError(#[from] gray_matter::Error),

    #[error("Could not parse date field: {0}")]
    DateParseError(#[from] chrono::ParseError),
}