use super::types::Post;
use crate::config::Config;
use chrono::Utc;
use sitewriter::{ChangeFreq, UrlEntry};
use std::fs;
use thiserror::Error;

/// Generates a sitemap from the list of posts
pub fn sitemap(config: &Config, posts: &[Post]) -> Result<(), SitemapError> {
    println!("Generating sitemap...");

    let mut urls = vec![UrlEntry {
        loc: format!("{}index.html", config.base_url).parse()?,
        changefreq: Some(ChangeFreq::Monthly),
        priority: Some(1.0),
        lastmod: Some(Utc::now()),
    }];

    let mut priority = 1.00;
    posts.iter().try_for_each(|p| -> Result<(), SitemapError> {
        urls.push(UrlEntry {
            loc: p.canonical.parse()?,
            changefreq: Some(ChangeFreq::Yearly),
            priority: Some(priority),
            lastmod: Some(p.published_at_raw),
        });

        priority *= 0.9;

        Ok(())
    })?;

    Ok(fs::write(
        format!("{}/sitemap.xml", config.build_path.display()),
        sitewriter::generate_str(&urls).as_bytes(),
    )?)
}

#[derive(Error, Debug)]
pub enum SitemapError {
    #[error("Unable to parse string to a valid url: {0}")]
    FsExtraError(#[from] rouille::url::ParseError),

    #[error("Unable to write sitemap file: {0}")]
    StdIoError(#[from] std::io::Error),
}
