mod parser;
mod rss;
mod sitemap;
mod tests;
mod types;

use crate::build::parser::markdown::MarkdownParser;
use crate::build::parser::Parser;
use crate::build::rss::rss;
use crate::build::sitemap::sitemap;
use crate::build::types::{BuildError, IndexPage, Post, TemplateData};
use crate::Config;
use fs_extra::dir::{copy, CopyOptions};
use glob::{glob, GlobError};
use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use tinytemplate::{format_unescaped, TinyTemplate};

/// Builds the blog to html
pub fn run() -> Result<(), BuildError> {
    println!("Building the blog");
    let config = &Config::new().expect("Unable to retrieve configuration");

    prepare_build_directory(&config.build_path).and_then(|_| {
        let mut posts = glob("./posts/*.md")?
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|entry| entry.map(|path| generate_file(config, &path)))
            .collect::<Result<Vec<Result<Post, BuildError>>, GlobError>>()?
            .into_iter()
            .collect::<Result<Vec<Post>, BuildError>>()?;

        // Order by descending publication date
        posts.sort_by_key(|p| -p.published_at_raw.timestamp());

        copy_static_assets(config)?;
        generate_index(config, &posts)?;
        sitemap(config, &posts)?;
        rss(
            config,
            &posts,
            &mut File::create(format!("{}/rss.xml", config.build_path.display()))?,
        )?;

        Ok(())
    })
}

fn prepare_build_directory(build_path: &Path) -> Result<(), BuildError> {
    if Path::new(build_path).exists() {
        fs::remove_dir_all(build_path)?;
    }

    Ok(fs::create_dir_all(build_path)?)
}

fn generate_file(config: &Config, path: &PathBuf) -> Result<Post, BuildError> {
    fs::read_to_string(path)
        .map_err(BuildError::StdIo)
        .and_then(|contents| Ok(MarkdownParser::new().parse(&contents)?))
        .and_then(|mut post| {
            post.canonical = get_canonical_url(&config.base_url, path)?;
            post.path = get_html_file_path(path)?;
            Ok(post)
        })
        .and_then(|post| generate_template(config, post))
        .and_then(|(html, post)| save(&config.build_path, post, html))
}

fn generate_template(config: &Config, post: Post) -> Result<(String, Post), BuildError> {
    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);
    let template = fs::read_to_string(format!("./themes/{}/post.html", &config.theme))?;
    tt.add_template("post", &template)?;

    Ok((
        tt.render(
            "post",
            &TemplateData {
                post: post.clone(),
                config: config.clone(),
            },
        )?,
        post,
    ))
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

fn save(build_path: &Path, post: Post, html: String) -> Result<Post, BuildError> {
    println!("Saving to {}...", post.path);

    fs::write(
        format!("{}/{}", build_path.display(), post.path),
        html.as_bytes(),
    )?;

    Ok(post)
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
    tt.add_template("index", &template)?;

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
