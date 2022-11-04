mod parser;
mod rss;
mod sitemap;
mod tests;
mod types;

use crate::build::parser::markdown::MarkdownParser;
use crate::build::parser::Parser;
use crate::build::rss::rss;
use crate::build::sitemap::sitemap;
use crate::build::types::{IndexPage, Post, TemplateData};
use crate::Config;
use anyhow::{anyhow, Context, Result};
use fs_extra::dir::{copy, CopyOptions};
use glob::{glob, GlobError};
use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use tinytemplate::{format_unescaped, TinyTemplate};

/// Builds the blog to html
pub fn run() -> Result<()> {
    println!("Building the blog");
    let config = &Config::new().context("Failed to retrieve the configuration")?;

    prepare_build_directory(&config.build_path).and_then(|_| {
        let mut posts = glob("posts/*.md")
            .context("Failed to read the md files at path posts/")?
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|entry| entry.map(|path| generate_file(config, &path)))
            .collect::<Result<Vec<Result<Post>>, GlobError>>()
            .context("Failed to read the markdown file")?
            .into_iter()
            .collect::<Result<Vec<Post>>>()
            .context("Failed to generate the post")?;

        // Order by descending publication date
        posts.sort_by_key(|p| -p.published_at_raw.timestamp());

        copy_static_assets(config).context("Failed to copy the static assets")?;
        generate_index(config, &posts).context("Failed to generate the index page")?;
        sitemap(config, &posts).context("Failed to generate the sitemap")?;
        rss(
            config,
            &posts,
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
    })
}

fn generate_file(config: &Config, path: &PathBuf) -> Result<Post> {
    fs::read_to_string(path)
        .map_err(|err| {
            anyhow!(
                "Failed to read the file at path {} with error {}",
                path.display(),
                err.to_string()
            )
        })
        .and_then(|contents| {
            MarkdownParser::new()
                .parse(&contents)
                .with_context(|| format!("Failed to parse the Markdown file at path {:?}", path))
        })
        .and_then(|mut post| {
            post.canonical = get_canonical_url(&config.base_url, path).with_context(|| {
                format!(
                    "Failed to generate the canonical url for the file at path {:?}",
                    path
                )
            })?;
            post.path = get_html_file_path(path).with_context(|| {
                format!(
                    "Failed to generate the html path for the file at path {:?}",
                    path
                )
            })?;
            Ok(post)
        })
        .and_then(|post| {
            generate_template(config, post.clone()).with_context(|| {
                format!(
                    "Failed to generate the template for the post at path {}",
                    post.path
                )
            })
        })
        .and_then(|(html, post)| {
            save(&config.build_path, post.clone(), html)
                .with_context(|| format!("Failed to save the file at path {}", post.path))
        })
}

fn generate_template(config: &Config, post: Post) -> Result<(String, Post)> {
    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);

    let template_path = &format!("./themes/{}/post.html", &config.theme);
    let template = fs::read_to_string(template_path)
        .with_context(|| format!("Failed to read the template file at path {}", template_path))?;
    tt.add_template("post", &template).with_context(|| {
        format!(
            "Failed to build the template for the post at path {}",
            post.path
        )
    })?;

    Ok((
        tt.render(
            "post",
            &TemplateData {
                post: post.clone(),
                config: config.clone(),
            },
        )
        .with_context(|| {
            format!(
                "Failed to render the template for the post at path {}",
                post.path
            )
        })?,
        post,
    ))
}

fn get_canonical_url(base_url: &str, path: &Path) -> Result<String> {
    get_file_name(path)
        .map(|filename| format!("{base_url}{}", str::replace(&filename, "md", "html")))
}

fn get_html_file_path(path: &Path) -> Result<String> {
    get_file_name(path).map(|filename| str::replace(&filename, "md", "html"))
}

fn get_file_name(path: &Path) -> Result<String> {
    Ok(path
        .file_name()
        .ok_or_else(|| anyhow!("Failed to retrieve filename at path {}", path.display(),))?
        .to_str()
        .ok_or_else(|| anyhow!("Empty filename at path {}", path.display()))?
        .to_string())
}

fn save(build_path: &Path, post: Post, html: String) -> Result<Post> {
    println!("Saving to {}...", post.path);

    fs::write(
        format!("{}/{}", build_path.display(), post.path),
        html.as_bytes(),
    )?;

    Ok(post)
}

fn copy_static_assets(config: &Config) -> Result<()> {
    println!("Copying static assets...");

    let favicon_from = &format!("./themes/{}/favicon.png", config.theme);
    let favicon_to = &format!("{}/favicon.png", config.build_path.display());
    fs::copy(favicon_from, favicon_to).with_context(|| {
        format!(
            "Failed to copy favicon from {} to {}",
            favicon_from, favicon_to
        )
    })?;

    let style_from = &format!("./themes/{}/style.css", config.theme);
    let style_to = &format!("{}/style.css", config.build_path.display());
    fs::copy(style_from, style_to).with_context(|| {
        format!(
            "Failed to copy stylesheet from {} to {}",
            style_from, style_to
        )
    })?;

    let images_from = "./static/images";
    let images_to = &config.build_path;
    copy(images_from, images_to, &CopyOptions::new()).with_context(|| {
        format!(
            "Failed to copy images from {} to {:?}",
            images_from, images_to
        )
    })?;

    Ok(())
}

fn generate_index(config: &Config, posts: &[Post]) -> Result<()> {
    println!("Generating the index page...");

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);

    let template_path = &format!("./themes/{}/index.html", &config.theme);
    let template = fs::read_to_string(template_path)
        .with_context(|| format!("Failed to read the template file at path {}", template_path))?;
    tt.add_template("index", &template)
        .context("Failed to build the index template ")?;

    let index = IndexPage {
        title: config.site_name.to_string(),
        description: config.description.to_string(),
        canonical: config.base_url.to_string(),
        locale: config.locale.to_string(),
        posts: posts.to_vec(),
        seo: config.seo.clone(),
        twitter: config.twitter.clone(),
    };

    let data = tt
        .render("index", &index)
        .context("Failed to render the index template")?;

    let index_path = &format!("{}/index.html", config.build_path.display());
    fs::write(index_path, data.as_bytes())
        .with_context(|| format!("Failed to write file at path {}", index_path))
}
