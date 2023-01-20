use crate::build::post::Posts;
use crate::config::Config;
use anyhow::{Context, Result};
use console::style;
use rss::{Channel, ChannelBuilder, Item, ItemBuilder};

// TODO: get rid of the rss crate and use a generic xml one
#[derive(Debug)]
pub struct Rss(Channel);

impl Rss {
    pub fn new(config: &Config, posts: &Posts) -> Self {
        Self(
            ChannelBuilder::default()
                .title(&config.site_name)
                .link(&config.base_url)
                .description(&config.description)
                .items(
                    posts
                        .get()
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
                .build(),
        )
    }

    pub fn write<T: std::io::Write>(&self, writer: T) -> Result<T> {
        println!("{}", style("Saving the RSS feed...").dim());

        self.0
            .write_to(writer)
            .context("Failed to consume the rss writer")
    }
}

#[cfg(test)]
mod build_tests {
    use crate::build::post::{Post, Posts};
    use crate::build::rss::Rss;
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
        let posts = Posts::new(vec![Post {
            title: "Lorem ipsum dolor sit amet".to_string(),
            description: "Morbi sollicitudin libero nisi, eu luctus quam tristique sed."
                .to_string(),
            published_at: "".to_string(),
            published_at_raw: Default::default(),
            content: "<p>Suspendisse vel nibh venenatis, convallis turpis nec, aliquet nibh. Vivamus rhoncus sapien nibh.</p>".to_string(),
            image: "https://placekitten.com/500/500".to_string(),
            canonical: "https://adriantombu.github.io/orion/lorem-ipsum-dolor-sit-amet".to_string(),
            path: "lorem-ipsum-dolor-sit-amet.html".to_string(),
            locale: "fr-FR".to_string(),
            draft: false
        }]);
        let mut result = Vec::new();

        let res = Rss::new(&config, &posts).write(&mut result);

        assert!(res.is_ok());
        assert!(!res.unwrap().is_empty());
    }
}
