use crate::build::parser::{ParsedData, Parser};
use crate::build::post::Post;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser as CmarkParser};

pub struct MarkdownParser {
    matter: Matter<YAML>,
}

impl Parser for MarkdownParser {
    fn new() -> Self {
        Self {
            matter: Matter::new(),
        }
    }

    fn parse(&self, text: &str, fallback_locale: &str) -> Result<Post> {
        self.matter
            .parse_with_struct::<ParsedData>(text)
            .ok_or_else(|| anyhow!("Missing required field"))
            .and_then(|parsed_contents| {
                let mut content = String::new();
                html::push_html(
                    &mut content,
                    CmarkParser::new_ext(&parsed_contents.content, Options::empty()),
                );

                let published_at_raw = DateTime::from_naive_utc_and_offset(
                    NaiveDateTime::parse_from_str(
                        &parsed_contents.data.published_at,
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .with_context(|| {
                        format!(
                            "Failed to parse date {} from string",
                            &parsed_contents.data.published_at
                        )
                    })?,
                    Utc,
                );

                Ok(Post {
                    title: parsed_contents.data.title,
                    description: parsed_contents.data.description,
                    published_at: published_at_raw.format("%Y-%m-%d").to_string(),
                    published_at_raw,
                    content,
                    image: parsed_contents.data.image.unwrap_or_default(),
                    locale: parsed_contents
                        .data
                        .locale
                        .unwrap_or_else(|| String::from(fallback_locale)),
                    draft: parsed_contents.data.draft.unwrap_or(false),
                    canonical: String::default(),
                    path: String::default(),
                })
            })
    }
}

#[cfg(test)]
mod build_tests {
    use crate::build::parser::markdown::MarkdownParser;
    use crate::build::parser::Parser;
    use crate::build::post::Post;
    use chrono::{DateTime, NaiveDateTime, Utc};

    #[test]
    fn test_markdown_parse_missing_required_field() {
        let contents = "---
    description: A blog generated with Orion
    published_at: 2020-01-01
    ---

    # Welcome to your blog";

        let res = MarkdownParser::new().parse(contents, "en_EN");

        assert!(res.is_err());
    }

    #[test]
    fn test_markdown_parse_missing_optional_field() {
        let contents = "---
title: Welcome to your blog!
description: A blog generated with Orion
published_at: 2020-01-01 12:34:56
---

# Welcome to your blog

This blog was [generated by Orion](https://github.com/adriantombu/orion), a simple static site generator aimed at making blogging easier!";
        let published_at_raw = DateTime::from_naive_utc_and_offset(
            NaiveDateTime::parse_from_str("2020-01-01 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap(),
            Utc,
        );

        let res = MarkdownParser::new().parse(contents, "en_EN");

        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Post {
                title: String::from("Welcome to your blog!"),
                description: String::from("A blog generated with Orion"),
                published_at: published_at_raw.format("%Y-%m-%d").to_string(),
                published_at_raw,
                content: String::from("<h1>Welcome to your blog</h1>\n<p>This blog was <a href=\"https://github.com/adriantombu/orion\">generated by Orion</a>, a simple static site generator aimed at making blogging easier!</p>\n"),
                image: String::new(),
                canonical: String::new(),
                path: String::new(),
                locale: String::from("en_EN"),
                draft: false
            }
        );
    }

    #[test]
    fn test_markdown_parse() {
        let contents = "---
title: Welcome to your blog!
description: A blog generated with Orion
published_at: 2020-01-01 12:34:56
locale: fr_FR
image: https://www.publicdomainpictures.net/pictures/220000/velka/orion-nebula.jpg
draft: true
---

# Welcome to your blog

This blog was [generated by Orion](https://github.com/adriantombu/orion), a simple static site generator aimed at making blogging easier!";
        let published_at_raw = DateTime::from_naive_utc_and_offset(
            NaiveDateTime::parse_from_str("2020-01-01 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap(),
            Utc,
        );

        let res = MarkdownParser::new().parse(contents, "en_EN");

        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Post {
                title: String::from("Welcome to your blog!"),
                description: String::from("A blog generated with Orion"),
                published_at: published_at_raw.format("%Y-%m-%d").to_string(),
                published_at_raw,
                content: String::from("<h1>Welcome to your blog</h1>\n<p>This blog was <a href=\"https://github.com/adriantombu/orion\">generated by Orion</a>, a simple static site generator aimed at making blogging easier!</p>\n"),
                image: String::from("https://www.publicdomainpictures.net/pictures/220000/velka/orion-nebula.jpg"),
                canonical: String::new(),
                path: String::new(),
                locale: String::from("fr_FR"),
                draft: true
            }
        );
    }
}
