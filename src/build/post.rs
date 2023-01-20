use chrono::{DateTime, Utc};
use serde::Serialize;
use std::slice::Iter;

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

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Posts(Vec<Post>);

impl Posts {
    pub fn new(posts: Vec<Post>) -> Self {
        Self(posts)
    }

    pub fn get(&self) -> Iter<Post> {
        self.0.iter()
    }

    pub fn sort_date_desc(&mut self) {
        self.0.sort_by_key(|p| -p.published_at_raw.timestamp());
    }
}
