use crate::config::{Seo, Twitter};
use crate::Config;
use chrono::{DateTime, Utc};
use serde::Serialize;

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
    pub locale: String,
    pub draft: bool,
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
