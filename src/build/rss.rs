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
    use chrono::DateTime;
    use std::path::PathBuf;

    #[test]
    fn test_rss() {
        // TODO: create a test helper?
        let config = Config {
            base_url: String::from("https://adriantombu.github.io/orion/"),
            site_name: String::from("Orion"),
            description: String::from("A simple static blog generator made with Rust"),
            theme: String::from("orion"),
            build_path: PathBuf::default(),
            locale: String::from("en_US"),
            seo: Seo {
                robots: String::from("index, follow"),
            },
            twitter: Twitter {
                creator: String::from("adriantombu"),
            },
        };
        let posts = Posts::new(vec![Post {
            title: String::from("Lorem ipsum dolor sit amet"),
            description: String::from("Morbi sollicitudin libero nisi, eu luctus quam tristique sed."),
            published_at: String::new(),
            published_at_raw: DateTime::default(),
            content: String::from("<p>Suspendisse vel nibh venenatis, convallis turpis nec, aliquet nibh. Vivamus rhoncus sapien nibh.</p>"),
            image: String::from("https://placekitten.com/500/500"),
            canonical: String::from("https://adriantombu.github.io/orion/lorem-ipsum-dolor-sit-amet"),
            path: String::from("lorem-ipsum-dolor-sit-amet.html"),
            locale: String::from("fr-FR"),
            draft: false
        }]);
        let mut result = Vec::new();

        let res = Rss::new(&config, &posts).write(&mut result);

        assert!(res.is_ok());
        assert!(!res.unwrap().is_empty());
    }
}
