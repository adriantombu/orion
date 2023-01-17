pub mod markdown;

use crate::build::types::Post;
use anyhow::Result;
use serde::Deserialize;

pub trait Parser {
    fn new() -> Self;
    fn parse(&self, text: &str, fallback_locale: &str) -> Result<Post>;
}

#[derive(Debug, Deserialize)]
pub struct ParsedData {
    pub title: String,
    pub description: String,
    pub published_at: String,
    pub image: Option<String>,
    pub content: Option<String>,
    pub locale: Option<String>,
}
