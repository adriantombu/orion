use super::types::Post;
use crate::config::Config;
use rss::{ChannelBuilder, Item, ItemBuilder};
use std::fs;
use thiserror::Error;

// Generate a RSS feed from the list of posts
pub fn rss(config: &Config, posts: &[Post]) -> Result<(), RssError> {
    println!("Generating RSS...");

    // TODO: limit to 5 last articles
    let items = posts
        .iter()
        .map(|p| {
            ItemBuilder::default()
                .title(p.title.to_string())
                .description(p.description.to_string())
                .link(p.canonical.to_string())
                .pub_date(p.published_at_raw.to_rfc2822())
                .content(p.content.to_string())
                .build()
        })
        .collect::<Vec<Item>>();

    let rss = ChannelBuilder::default()
        .title(&config.site_name)
        .link(&config.base_url)
        .description(&config.description)
        .items(items)
        .build();

    Ok(fs::write(
        format!("{}/rss.xml", config.build_path.display()),
        rss.to_string(),
    )?)
}

#[derive(Error, Debug)]
pub enum RssError {
    #[error("Unable to write sitemap file: {0}")]
    StdIoError(#[from] std::io::Error),
}
