mod tests;
mod types;

use crate::build::types::BuildError;
use glob::glob;

use crate::parser::markdown::MarkdownParser;
use crate::parser::{ParsedData, Parser};
use crate::Config;
use std::fs;
use std::path::{Path, PathBuf};
use tinytemplate::{format_unescaped, TinyTemplate};

/// Builds the blog to html
pub fn run(_config: &Config) -> Result<(), BuildError> {
    println!("Building the blog");

    glob("./articles/*.md")?.try_for_each(|entry| entry.map(|path| generate_file(&path))?)
}

fn generate_file(path: &PathBuf) -> Result<(), BuildError> {
    fs::read_to_string(path)
        .map_err(BuildError::StdIoError)
        .and_then(|contents| Ok(MarkdownParser::new().parse(&contents)?))
        .and_then(|data| generate_template(&data))
        .and_then(|html| save(get_html_file_path(path)?, html))
    // TODO: reset build directory
    // TODO: generate pagination
    // TODO: generate index page
    // TODO: generate sitemap
    // TODO: generate rss
    // TODO: copy assets
}

fn generate_template(data: &ParsedData) -> Result<String, BuildError> {
    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);
    // TODO: retrieve theme path from config
    let template = fs::read_to_string("./themes/otso/article.html")?;
    tt.add_template("article", &*template)?;

    Ok(tt.render("article", &data)?)
}

fn get_html_file_path(path: &Path) -> Result<String, BuildError> {
    println!("{:?}", path);

    path.file_name()
        .ok_or(BuildError::EmptyFilenameError)?
        .to_str()
        .ok_or(BuildError::EmptyFilenameError)
        // TODO: retrive output path from config
        .map(|filename| format!("./public/{}", str::replace(filename, "md", "html")))
}

fn save(file_path: String, html: String) -> Result<(), BuildError> {
    println!("Saving to {}", file_path);

    // TODO: retrive output path from config
    fs::create_dir_all("./public/")?;
    Ok(fs::write(file_path, html.as_bytes())?)
}
