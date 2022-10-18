pub mod markdown;

use serde::Serialize;
use thiserror::Error;

pub trait Parser {
    fn new() -> Self;
    fn parse(&self, text: &str) -> Result<ParsedData, ParserError>;
}

// TODO: handle more values
#[derive(Debug, Serialize, Default)]
pub struct ParsedData {
    pub title: String,
    pub description: String,
    pub published_at: String,
    pub content: String,
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Empty value for parsed file")]
    EmptyValueError,

    #[error("Could not parse markdown field: {0}")]
    MarkdownError(#[from] gray_matter::Error),
}
