mod tests;
mod types;

use crate::build::types::BuildError;
use crate::parser::markdown::MarkdownParser;
use crate::parser::{ParsedData, Parser};
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
        glob("./articles/*.md")?
            .try_for_each(|entry| entry.map(|path| generate_file(config, &path))?)
            .and_then(|_| copy_static_assets(config))

        // TODO: generate pagination
        // TODO: generate index page
        // TODO: generate sitemap
        // TODO: generate rss
    })
}

fn prepare_build_directory(build_path: &Path) -> Result<(), BuildError> {
    if Path::new(build_path).exists() {
        fs::remove_dir_all(build_path)?;
    }

    Ok(fs::create_dir_all(build_path)?)
}

fn generate_file(config: &Config, path: &PathBuf) -> Result<(), BuildError> {
    fs::read_to_string(path)
        .map_err(BuildError::StdIoError)
        .and_then(|contents| Ok(MarkdownParser::new().parse(&contents)?))
        .and_then(|data| generate_template(&config.theme, &data))
        .and_then(|html| save(get_html_file_path(&config.build_path, path)?, html))
}

fn generate_template(theme: &str, data: &ParsedData) -> Result<String, BuildError> {
    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);
    let template = fs::read_to_string(format!("./themes/{theme}/article.html"))?;
    tt.add_template("article", &*template)?;

    Ok(tt.render("article", &data)?)
}

fn get_html_file_path(build_path: &Path, path: &Path) -> Result<String, BuildError> {
    println!("{:?}", path);

    path.file_name()
        .ok_or(BuildError::EmptyFilenameError)?
        .to_str()
        .ok_or(BuildError::EmptyFilenameError)
        .map(|filename| {
            format!(
                "{}/{}",
                build_path.display(),
                str::replace(filename, "md", "html")
            )
        })
}

fn save(file_path: String, html: String) -> Result<(), BuildError> {
    println!("Saving to {}...", file_path);

    Ok(fs::write(file_path, html.as_bytes())?)
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
