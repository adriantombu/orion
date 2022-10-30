use super::{rss::RssError, sitemap::SitemapError};
use crate::build::parser::ParserError;
use crate::config::{Seo, Twitter};
use crate::Config;
use chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("{0}")]
    Pattern(#[from] glob::PatternError),

    #[error("{0}")]
    Glob(#[from] glob::GlobError),

    #[error("{0}")]
    StdIo(#[from] std::io::Error),

    #[error("{0}")]
    Template(#[from] tinytemplate::error::Error),

    #[error("{0}")]
    FsExtra(#[from] fs_extra::error::Error),

    #[error("Filename is empty")]
    EmptyFilename,

    #[error("Unable to parse file: {0}")]
    Parser(#[from] ParserError),

    #[error("Unable to generate sitemap: {0}")]
    Sitemap(#[from] SitemapError),

    #[error("Unable to generate rss feed: {0}")]
    Rss(#[from] RssError),
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Post {
    pub title: String,
    pub description: String,
    pub published_at: String,
    pub published_at_raw: DateTime<Utc>,
    pub content: String,
    pub image: String,
    pub canonical: String,
    pub path: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct IndexPage {
    pub title: String,
    pub description: String,
    pub canonical: String,
    pub locale: String,
    pub posts: Vec<Post>,
    pub seo: Seo,
    pub twitter: Twitter,
}

#[derive(Debug, Serialize, Clone)]
pub struct TemplateData {
    pub post: Post,
    pub config: Config,
}
