mod parser;
mod rss;
mod sitemap;
mod tests;
mod types;

use crate::build::parser::markdown::MarkdownParser;
use crate::build::parser::Parser;
use crate::build::rss::rss;
use crate::build::sitemap::sitemap;
use crate::build::types::{BuildError, IndexPage, Post};
use crate::Config;
use fs_extra::dir::{copy, CopyOptions};
use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};
use tinytemplate::{format_unescaped, TinyTemplate};

/// Builds the blog to html
pub fn run(config: &Config) -> Result<(), BuildError> {
    println!("Building the blog");

    prepare_build_directory(&config.build_path).and_then(|_| {
        let mut posts: Vec<Post> = vec![];

        glob("./posts/*.md")?
            .try_for_each(|entry| entry.map(|path| generate_file(config, &mut posts, &path))?)
            .and_then(|_| copy_static_assets(config))?;

        // Order by descending publication date
        posts.sort_by_key(|p| -p.published_at_raw.timestamp());

        generate_index(config, &posts)?;
        sitemap(config, &posts)?;
        rss(config, &posts)?;

        Ok(())
    })
}

fn prepare_build_directory(build_path: &Path) -> Result<(), BuildError> {
    if Path::new(build_path).exists() {
        fs::remove_dir_all(build_path)?;
    }

    Ok(fs::create_dir_all(build_path)?)
}

fn generate_file(config: &Config, posts: &mut Vec<Post>, path: &PathBuf) -> Result<(), BuildError> {
    fs::read_to_string(path)
        .map_err(BuildError::StdIo)
        .and_then(|contents| Ok(MarkdownParser::new().parse(&contents)?))
        .and_then(|data| {
            let post = Post {
                title: data.title,
                description: data.description,
                published_at: data.published_at.format("%Y-%m-%d").to_string(),
                published_at_raw: data.published_at,
                content: data.content,
                image: data.image,
                canonical: get_canonical_url(&config.base_url, path)?,
                locale: config.locale.to_string(),
                sitename: config.site_name.clone(),
                path: get_html_file_path(path)?,
                seo: config.seo.clone(),
                twitter: config.twitter.clone(),
            };
            posts.push(post.clone());
            Ok(post)
        })
        .and_then(|post| generate_template(&config.theme, post))
        .and_then(|(html, post)| save(&config.build_path, post.path, html))
}

fn generate_template(theme: &str, post: Post) -> Result<(String, Post), BuildError> {
    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);
    let template = fs::read_to_string(format!("./themes/{theme}/post.html"))?;
    tt.add_template("post", &*template)?;

    Ok((tt.render("post", &post)?, post))
}

fn get_canonical_url(base_url: &str, path: &Path) -> Result<String, BuildError> {
    get_file_name(path)
        .map(|filename| format!("{base_url}{}", str::replace(&filename, "md", "html")))
}

fn get_html_file_path(path: &Path) -> Result<String, BuildError> {
    get_file_name(path).map(|filename| str::replace(&filename, "md", "html"))
}

fn get_file_name(path: &Path) -> Result<String, BuildError> {
    Ok(path
        .file_name()
        .ok_or(BuildError::EmptyFilename)?
        .to_str()
        .ok_or(BuildError::EmptyFilename)?
        .to_string())
}

fn save(build_path: &Path, file_path: String, html: String) -> Result<(), BuildError> {
    println!("Saving to {}...", file_path);

    Ok(fs::write(
        format!("{}/{}", build_path.display(), file_path),
        html.as_bytes(),
    )?)
}

fn copy_static_assets(config: &Config) -> Result<(), BuildError> {
    println!("Copying static assets...");

    fs::copy(
        format!("./themes/{}/favicon.png", config.theme),
        format!("{}/favicon.png", config.build_path.display()),
    )?;

    fs::copy(
        format!("./themes/{}/style.css", config.theme),
        format!("{}/style.css", config.build_path.display()),
    )?;

    copy("./static/images", &config.build_path, &CopyOptions::new())?;

    Ok(())
}

fn generate_index(config: &Config, posts: &[Post]) -> Result<(), BuildError> {
    println!("Generating index page...");

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);
    let template = fs::read_to_string(format!("./themes/{}/index.html", config.theme))?;
    tt.add_template("index", &*template)?;

    let index = IndexPage {
        title: config.site_name.to_string(),
        description: config.description.to_string(),
        canonical: config.base_url.to_string(),
        locale: config.locale.to_string(),
        posts: posts.to_vec(),
        seo: config.seo.clone(),
        twitter: config.twitter.clone(),
    };

    let data = tt.render("index", &index)?;

    Ok(fs::write(
        &format!("{}/index.html", config.build_path.display()),
        data.as_bytes(),
    )?)
}
