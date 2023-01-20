use crate::build::post::Posts;
use crate::config::Config;
use anyhow::{Context, Result};
use chrono::Utc;
use console::style;
use sitewriter::{ChangeFreq, UrlEntry};

// TODO: get rid of the sitewriter crate and use a generic xml one
#[derive(Debug)]
pub struct Sitemap(String);

impl Sitemap {
    pub fn new(config: &Config, posts: &Posts) -> Result<Sitemap> {
        let mut urls = vec![UrlEntry {
            loc: format!("{}index.html", config.base_url)
                .parse()
                .with_context(|| {
                    format!("Failed to parse the config base_url {}", config.base_url)
                })?,
            changefreq: Some(ChangeFreq::Monthly),
            priority: Some(1.0),
            lastmod: Some(Utc::now()),
        }];

        let mut priority = 1.00;
        posts
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

        Ok(Sitemap(sitewriter::generate_str(&urls)))
    }

    pub fn write<T: std::io::Write>(&self, mut writer: T) -> Result<usize> {
        println!("{}", style("Saving the sitemap...").dim());

        writer
            .write(self.0.as_bytes())
            .context("Failed to consume the sitemap writer")
    }
}
