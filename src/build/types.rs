use crate::build::parser::ParserError;
use crate::config::{Seo, Twitter};
use chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;

use super::{rss::RssError, sitemap::SitemapError};

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("{0}")]
    PatternError(#[from] glob::PatternError),

    #[error("{0}")]
    GlobError(#[from] glob::GlobError),

    #[error("{0}")]
    StdIoError(#[from] std::io::Error),

    #[error("{0}")]
    TemplateError(#[from] tinytemplate::error::Error),

    #[error("{0}")]
    FsExtraError(#[from] fs_extra::error::Error),

    #[error("Filename is empty")]
    EmptyFilenameError,

    #[error("Unable to parse file: {0}")]
    ParserError(#[from] ParserError),

    #[error("Unable to generate sitemap: {0}")]
    SitemapError(#[from] SitemapError),

    #[error("Unable to generate rss feed: {0}")]
    RssError(#[from] RssError),
}

#[derive(Debug, Serialize, Clone)]
pub struct Post {
    pub title: String,
    pub description: String,
    pub published_at: String,
    pub published_at_raw: DateTime<Utc>,
    pub content: String,
    pub image: String,
    pub canonical: String,
    pub locale: String,
    pub sitename: String,
    pub path: String,
    pub seo: Seo,
    pub twitter: Twitter,
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
