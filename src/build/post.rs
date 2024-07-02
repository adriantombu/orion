use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashSet;
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
    pub categories: Vec<String>,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Posts {
    pub categories: HashSet<String>,
    posts: Vec<Post>,
}

impl Posts {
    pub fn new(posts: Vec<Post>) -> Self {
        let mut categories = HashSet::new();
        for p in &posts {
            p.categories.iter().for_each(|c| {
                categories.insert(c.to_string());
            });
        }

        Self { categories, posts }
    }

    pub fn get(&self) -> Iter<Post> {
        self.posts.iter()
    }

    pub fn filter(&self, category: &String) -> Self {
        Self {
            categories: self.categories.clone(),
            posts: self
                .get()
                .filter(|p| p.categories.contains(category))
                .cloned()
                .collect::<Vec<_>>(),
        }
    }

    pub fn sort_date_desc(&mut self) {
        self.posts.sort_by_key(|p| -p.published_at_raw.timestamp());
    }
}
