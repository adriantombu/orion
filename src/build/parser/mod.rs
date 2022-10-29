pub mod markdown;

use crate::build::types::Post;
use serde::Deserialize;
use thiserror::Error;

pub trait Parser {
    fn new() -> Self;
    fn parse(&self, text: &str) -> Result<Post, ParserError>;
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ParserError {
    #[error("A required field is missing")]
    MissingRequiredField,

    #[error("Could not parse markdown field: {0}")]
    Markdown(#[from] gray_matter::Error),

    #[error("Could not parse date field: {0}")]
    DateParse(#[from] chrono::ParseError),
}

#[derive(Debug, Deserialize)]
pub struct ParsedData {
    pub title: String,
    pub description: String,
    pub published_at: String,
    pub image: Option<String>,
    pub content: Option<String>,
}
