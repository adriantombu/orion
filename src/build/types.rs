use crate::build::post::{Post, Posts};
use crate::config::{Seo, Twitter};
use crate::Config;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct IndexPage<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub canonical: &'a str,
    pub locale: &'a str,
    pub posts: &'a Posts,
    pub seo: &'a Seo,
    pub twitter: &'a Twitter,
}

#[derive(Debug, Serialize, Clone)]
pub struct TemplateData<'a> {
    pub post: &'a Post,
    pub config: &'a Config,
}
