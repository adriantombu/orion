mod tests;
mod types;

use crate::build::types::{BuildError, Data};
use glob::glob;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::{Path, PathBuf};

pub fn run() -> Result<(), BuildError> {
    println!("Building the blog");

    glob("../blog/articles/*.md")?
        .try_for_each(|entry| entry.map(|path| generate_file(&Matter::<YAML>::new(), path))?)
}

fn generate_file(matter: &Matter<YAML>, path: PathBuf) -> Result<(), BuildError> {
    let (data, content) = parse_markdown(matter, &path)?;
    let html = generate_template(data, content)?;
    let file_path = get_html_file_path(&path)?;

    save(file_path, html)
}

fn parse_markdown(matter: &Matter<YAML>, path: &PathBuf) -> Result<(Data, String), BuildError> {
    let article = matter.parse(&fs::read_to_string(path)?);
    let data = article.data.ok_or(BuildError::Parse).and_then(|fm| {
        Ok(Data {
            title: fm["title"].as_string()?,
            description: fm["description"].as_string()?,
            published_at: fm["published_at"].as_string()?,
        })
    })?;

    let mut content = String::new();
    html::push_html(
        &mut content,
        Parser::new_ext(&article.content, Options::empty()),
    );

    Ok((data, content))
}

// TODO: generate template
fn generate_template(_data: Data, content: String) -> Result<String, BuildError> {
    Ok(content)
}

fn get_html_file_path(path: &Path) -> Result<String, BuildError> {
    println!("{:?}", path);

    path.file_name()
        .ok_or(BuildError::EmptyFilename)?
        .to_str()
        .ok_or(BuildError::EmptyFilename)
        .map(|filename| format!("../blog/public/{}", str::replace(filename, "md", "html")))
}

fn save(file_path: String, html: String) -> Result<(), BuildError> {
    println!("Saving to {}", file_path);

    fs::create_dir_all("../blog/public/")?;
    Ok(fs::write(file_path, html.as_bytes())?)
}
