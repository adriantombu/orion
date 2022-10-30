use super::types::Post;
use crate::config::Config;
use rss::{ChannelBuilder, Item, ItemBuilder};
use thiserror::Error;

// Generate a RSS feed from the list of posts
pub fn rss<T: std::io::Write>(config: &Config, posts: &[Post], writer: T) -> Result<T, RssError> {
    println!("Generating RSS...");

    Ok(ChannelBuilder::default()
        .title(&config.site_name)
        .link(&config.base_url)
        .description(&config.description)
        .items(
            posts
                .iter()
                .take(5)
                .map(|p| {
                    ItemBuilder::default()
                        .title(p.title.to_string())
                        .description(p.description.to_string())
                        .link(p.canonical.to_string())
                        .pub_date(p.published_at_raw.to_rfc2822())
                        .content(p.content.to_string())
                        .build()
                })
                .collect::<Vec<Item>>(),
        )
        .build()
        .write_to(writer)?)
}

#[derive(Error, Debug)]
pub enum RssError {
    #[error("Unable to write sitemap file: {0}")]
    StdIo(#[from] std::io::Error),

    #[error("Unable to write sitemap file: {0}")]
    RssCrate(#[from] rss::Error),
}

#[cfg(test)]
mod build_tests {
    use crate::build::rss::rss;
    use crate::build::types::Post;
    use crate::config::{Config, Seo, Twitter};

    #[test]
    fn test_rss() {
        // TODO: create a test helper?
        let config = Config {
            base_url: "https://adriantombu.github.io/orion/".to_string(),
            site_name: "Orion".to_string(),
            description: "A simple static blog generator made with Rust".to_string(),
            theme: "orion".to_string(),
            build_path: Default::default(),
            locale: "en_US".to_string(),
            seo: Seo {
                robots: "index, follow".to_string(),
            },
            twitter: Twitter {
                creator: "adriantombu".to_string(),
            },
        };
        let posts = vec![Post {
            title: "Lorem ipsum dolor sit amet".to_string(),
            description: "Morbi sollicitudin libero nisi, eu luctus quam tristique sed."
                .to_string(),
            published_at: "".to_string(),
            published_at_raw: Default::default(),
            content: "<p>Suspendisse vel nibh venenatis, convallis turpis nec, aliquet nibh. Vivamus rhoncus sapien nibh.</p>".to_string(),
            image: "https://placekitten.com/500/500".to_string(),
            canonical: "https://adriantombu.github.io/orion/lorem-ipsum-dolor-sit-amet".to_string(),
            path: "lorem-ipsum-dolor-sit-amet.html".to_string(),
        }];
        let mut result = Vec::new();

        let res = rss(&config, &posts, &mut result);

        assert!(res.is_ok());
        assert!(!res.unwrap().is_empty());
    }
}
