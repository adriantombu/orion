use crate::build::post::Posts;
use crate::config::Config;
use anyhow::{Context, Result};
use chrono::Utc;
use sitewriter::{ChangeFreq, UrlEntry};
use std::fs;

// TODO: get rid of the sitewriter crate and use a generic xml one
#[derive(Debug)]
pub struct Sitemap<'a> {
    config: &'a Config,
    posts: &'a Posts,
    urls: Option<Vec<UrlEntry>>,
}

impl Sitemap<'_> {
    pub fn new<'a>(config: &'a Config, posts: &'a Posts) -> Sitemap<'a> {
        Sitemap {
            config,
            posts,
            urls: None,
        }
    }

    pub fn generate(&self) -> Result<Self> {
        let mut urls = vec![UrlEntry {
            loc: format!("{}index.html", self.config.base_url)
                .parse()
                .with_context(|| {
                    format!(
                        "Failed to parse the config base_url {}",
                        self.config.base_url
                    )
                })?,
            changefreq: Some(ChangeFreq::Monthly),
            priority: Some(1.0),
            lastmod: Some(Utc::now()),
        }];

        let mut priority = 1.00;
        self.posts
            .get()
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

        Ok(Sitemap {
            config: self.config,
            posts: self.posts,
            urls: Some(urls),
        })
    }

    pub fn save_to_file(self) -> Result<()> {
        let build_path = self.config.build_path.display();

        fs::write(
            format!("{}/sitemap.xml", build_path),
            sitewriter::generate_str(&self.urls.unwrap_or_default()).as_bytes(),
        )
        .with_context(|| {
            format!(
                "Failed to write the sitemap at path {}/sitemap.xml",
                build_path
            )
        })
    }
}
