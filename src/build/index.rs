use crate::build::post::Posts;
use crate::config::{Seo, Twitter};
use crate::Config;
use anyhow::{Context, Result};
use console::style;
use serde::Serialize;
use std::fs;
use tinytemplate::{format_unescaped, TinyTemplate};

#[derive(Debug, Serialize, Clone)]
pub struct Index<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub canonical: &'a str,
    pub locale: &'a str,
    pub posts: &'a Posts,
    pub seo: &'a Seo,
    pub twitter: &'a Twitter,
}

impl Index<'_> {
    pub fn new<'a>(config: &'a Config, posts: &'a Posts) -> Index<'a> {
        Index {
            title: &config.site_name,
            description: &config.description,
            canonical: &config.base_url,
            locale: &config.locale,
            posts,
            seo: &config.seo,
            twitter: &config.twitter,
        }
    }

    pub fn write<T: std::io::Write>(&self, mut writer: T, config: &Config) -> Result<usize> {
        println!("{}", style("Generating the index page...").dim());

        let mut tt = TinyTemplate::new();
        tt.set_default_formatter(&format_unescaped);
        let template_path = &format!("./themes/{}/index.html", config.theme);
        let template = fs::read_to_string(template_path)
            .with_context(|| format!("Failed to read the template file at path {template_path}"))?;
        tt.add_template("index", &template)
            .context("Failed to build the index template ")?;

        let data = tt
            .render("index", &self)
            .context("Failed to render the index template")?;

        writer
            .write(data.as_bytes())
            .context("Failed to consume the sitemap writer")
    }
}
