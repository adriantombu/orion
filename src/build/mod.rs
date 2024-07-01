mod assets;
mod index;
mod parser;
mod post;
mod rss;
mod sitemap;
mod tests;
mod types;

use crate::build::assets::Assets;
use crate::build::index::Index;
use crate::build::parser::markdown::MarkdownParser;
use crate::build::parser::Parser;
use crate::build::post::{Post, Posts};
use crate::build::rss::Rss;
use crate::build::sitemap::Sitemap;
use crate::build::types::TemplateData;
use crate::Config;
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use console::style;
use glob::glob;
use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use tinytemplate::{format_unescaped, TinyTemplate};

/// Builds the blog to html
pub fn run() -> Result<()> {
    println!("{}", style("Building the blog...").cyan());

    let config = &Config::new().context("Failed to retrieve the configuration")?;

    prepare_build_directory(&config.build_path).and_then(|()| {
        let files = glob("posts/*.md")
            .context("Failed to read the md files at path posts/")?
            .par_bridge()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        let mut posts = Posts::new(
            files
                .into_par_iter()
                .map(|path| generate_file(config, &path))
                .collect::<Result<Vec<_>>>()
                .context("Failed to generate the post")?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
        );
        posts.sort_date_desc();

        for category in &posts.categories {
            Index::new(config, &posts.filter(category))
                .write(
                    &mut File::create(format!(
                        "{}/category/{}.html",
                        config.build_path.display(),
                        category
                    ))
                    .context("Failed to generate the category file")?,
                    config,
                )
                .context("Failed to generate the category page")?;
        }

        Assets::copy(config).context("Failed to copy the static assets")?;
        Index::new(config, &posts)
            .write(
                &mut File::create(format!("{}/index.html", config.build_path.display()))
                    .context("Failed to create the index file")?,
                config,
            )
            .context("Failed to generate the index page")?;
        Sitemap::new(config, &posts)?.write(
            &mut File::create(format!("{}/sitemap.xml", config.build_path.display()))
                .context("Failed to generate the sitemap")?,
        )?;
        Rss::new(config, &posts).write(
            &mut File::create(format!("{}/rss.xml", config.build_path.display()))
                .context("Failed to generate the RSS feed")?,
        )?;

        Ok(())
    })
}

fn prepare_build_directory(build_path: &Path) -> Result<()> {
    if Path::new(build_path).exists() {
        fs::remove_dir_all(build_path).with_context(|| {
            format!(
                "Failed to delete the directory at path {}",
                build_path.display()
            )
        })?;
    }

    fs::create_dir_all(build_path).with_context(|| {
        format!(
            "Failed to create the directory at path {}",
            build_path.display()
        )
    })?;

    fs::create_dir_all(format!("{}/category", build_path.display())).with_context(|| {
        format!(
            "Failed to create the directory at path {}",
            build_path.display()
        )
    })
}

fn generate_file(config: &Config, path: &PathBuf) -> Result<Option<Post>> {
    let mut post = fs::read_to_string(path)
        .map_err(|err| {
            anyhow!(
                "Failed to read the file at path {} with error {}",
                path.display(),
                err.to_string()
            )
        })
        .and_then(|contents| {
            MarkdownParser::new()
                .parse(&contents, &config.locale)
                .with_context(|| format!("Failed to parse the Markdown file at path {path:?}"))
        })?;

    if post.draft || post.published_at_raw > Utc::now() {
        println!(
            "{}",
            style(format!("\"{}\" is a draft, skipping...", &post.title))
        );

        return Ok(None);
    }

    post.canonical = get_canonical_url(&config.base_url, path).with_context(|| {
        format!("Failed to generate the canonical url for the file at path {path:?}")
    })?;
    post.path = get_html_file_path(path).with_context(|| {
        format!("Failed to generate the html path for the file at path {path:?}")
    })?;

    generate_template(config, &post)
        .with_context(|| {
            format!(
                "Failed to generate the template for the post at path {}",
                post.path
            )
        })
        .and_then(|html| {
            save(&config.build_path, &post, &html)
                .with_context(|| format!("Failed to save the file at path {}", post.path))
        })?;

    Ok(Some(post))
}

fn generate_template(config: &Config, post: &Post) -> Result<String> {
    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);

    let template_path = &format!("./themes/{}/post.html", &config.theme);
    let template = fs::read_to_string(template_path)
        .with_context(|| format!("Failed to read the template file at path {template_path}"))?;
    tt.add_template("post", &template).with_context(|| {
        format!(
            "Failed to build the template for the post at path {}",
            post.path
        )
    })?;

    tt.render("post", &TemplateData { post, config })
        .with_context(|| {
            format!(
                "Failed to render the template for the post at path {}",
                post.path
            )
        })
}

fn get_canonical_url(base_url: &str, path: &Path) -> Result<String> {
    get_file_name(path)
        .map(|filename| format!("{base_url}{}", str::replace(&filename, "md", "html")))
}

fn get_html_file_path(path: &Path) -> Result<String> {
    get_file_name(path).map(|filename| str::replace(&filename, "md", "html"))
}

fn get_file_name(path: &Path) -> Result<String> {
    Ok(String::from(
        path.file_name()
            .ok_or_else(|| anyhow!("Failed to retrieve filename at path {}", path.display(),))?
            .to_str()
            .ok_or_else(|| anyhow!("Empty filename at path {}", path.display()))?,
    ))
}

fn save(build_path: &Path, post: &Post, html: &str) -> Result<()> {
    println!("{}", style(format!("Saving to {}...", &post.path)).dim());

    fs::write(
        format!("{}/{}", build_path.display(), post.path),
        html.as_bytes(),
    )?;

    Ok(())
}
