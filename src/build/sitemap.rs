use crate::build::post::Posts;
use crate::config::Config;
use anyhow::{Context, Result};
use chrono::{SecondsFormat, Utc};
use console::style;
use quick_xml::events::{BytesDecl, BytesText, Event};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Debug)]
pub struct Sitemap(String);

impl Sitemap {
    pub fn new(config: &Config, posts: &Posts) -> Result<Sitemap> {
        let mut urls = vec![Url {
            loc: format!("{}index.html", config.base_url)
                .parse()
                .with_context(|| {
                    format!("Failed to parse the config base_url {}", config.base_url)
                })?,
            changefreq: Some("monthly".to_string()),
            priority: Some(1.0),
            lastmod: Some(Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)),
        }];

        let mut priority = 1.00;
        posts
            .get()
            .try_for_each(|p| -> Result<()> {
                urls.push(Url {
                    loc: p.canonical.parse().with_context(|| {
                        format!("Failed to parse the canonical url {}", p.canonical)
                    })?,
                    changefreq: Some("yearly".to_string()),
                    priority: Some(priority),
                    lastmod: Some(
                        p.published_at_raw
                            .to_rfc3339_opts(SecondsFormat::Secs, true),
                    ),
                });

                priority *= 0.9;

                Ok(())
            })
            .context("Failed to generate the list of posts")?;

        Ok(Sitemap(Sitemap::generate_str(&urls)?))
    }

    pub fn write<T: std::io::Write>(&self, mut writer: T) -> Result<usize> {
        println!("{}", style("Saving the sitemap...").dim());

        writer
            .write(self.0.as_bytes())
            .context("Failed to consume the sitemap writer")
    }

    fn generate_str(urls: &[Url]) -> Result<String> {
        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);
        writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
            .context("Failed to generate the xml header element")?;

        writer
            .create_element("urlset")
            .with_attribute(("xmlns", "https://www.sitemaps.org/schemas/sitemap/0.9"))
            .write_inner_content::<_, quick_xml::Error>(|writer| {
                urls.iter()
                    .try_for_each(|url| -> Result<()> {
                        writer
                            .create_element("url")
                            .write_inner_content::<_, quick_xml::Error>(|writer| {
                                writer
                                    .create_element("loc")
                                    .write_text_content(BytesText::new(&url.loc))?;

                                if url.lastmod.is_some() {
                                    writer.create_element("lastmod").write_text_content(
                                        BytesText::new(url.lastmod.as_ref().unwrap()),
                                    )?;
                                }

                                if url.priority.is_some() {
                                    writer.create_element("priority").write_text_content(
                                        BytesText::new(&format!("{:.2}", url.priority.unwrap())),
                                    )?;
                                }

                                if url.changefreq.is_some() {
                                    writer.create_element("changefreq").write_text_content(
                                        BytesText::new(url.changefreq.as_ref().unwrap()),
                                    )?;
                                }

                                Ok(())
                            })
                            .context("Failed to generate the xml url element")?;

                        Ok(())
                    })
                    .unwrap();

                Ok(())
            })?;

        Ok(std::str::from_utf8(&writer.into_inner().into_inner())
            .context("Failed to convert the writer to string")?
            .to_string())
    }
}

#[derive(Debug)]
struct Url {
    pub loc: String,
    pub lastmod: Option<String>,
    pub priority: Option<f32>,
    pub changefreq: Option<String>,
}
