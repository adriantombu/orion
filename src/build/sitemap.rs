use super::types::Post;
use crate::config::Config;
use anyhow::{Context, Result};
use chrono::Utc;
use console::style;
use sitewriter::{ChangeFreq, UrlEntry};
use std::fs;

// TODO: get rid of the sitewriter crate and use a generic xml one
/// Generates a sitemap from the list of posts
pub fn sitemap(config: &Config, posts: &[Post]) -> Result<()> {
    println!("{}", style("Generating the sitemap...").dim());

    let mut urls = vec![UrlEntry {
        loc: format!("{}index.html", config.base_url)
            .parse()
            .with_context(|| format!("Failed to parse the config base_url {}", config.base_url))?,
        changefreq: Some(ChangeFreq::Monthly),
        priority: Some(1.0),
        lastmod: Some(Utc::now()),
    }];

    let mut priority = 1.00;
    posts
        .iter()
        .try_for_each(|p| -> Result<()> {
            urls.push(UrlEntry {
                loc: p.canonical.parse().with_context(|| {
                    format!("Failed to parse the canonical url {}", p.canonical)
                })?,
                changefreq: Some(ChangeFreq::Yearly),
                priority: Some(priority),
                lastmod: Some(p.published_at_raw),
            });

            priority *= 0.9;

            Ok(())
        })
        .context("Failed to generate the list of posts")?;

    fs::write(
        format!("{}/sitemap.xml", config.build_path.display()),
        sitewriter::generate_str(&urls).as_bytes(),
    )
    .with_context(|| {
        format!(
            "Failed to write the sitemap at path {}/sitemap.xml",
            config.build_path.display()
        )
    })
}
