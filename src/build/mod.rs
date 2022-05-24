mod tests;
mod types;

use crate::build::types::Data;
use glob::glob;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser};
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

pub fn run() {
    println!("Building the blog");

    let matter = Matter::<YAML>::new();

    for entry in glob("../blog/articles/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => generate_file(&matter, path),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn generate_file(matter: &Matter<YAML>, mut path: PathBuf) {
    let (data, content) = parse_markdown(matter, &path);
    let html = generate_template(data, content);
    let file_path = get_html_file_path(&mut path);

    save(file_path, html).unwrap();
}

fn parse_markdown(matter: &Matter<YAML>, path: &PathBuf) -> (Data, String) {
    let content = fs::read_to_string(path).unwrap();
    let article = matter.parse(&content);

    let fm = article.data.unwrap();
    let data = Data {
        title: fm["title"].as_string().unwrap(),
        description: fm["description"].as_string().unwrap(),
        published_at: fm["published_at"].as_string().unwrap(),
    };

    let parser = Parser::new_ext(&article.content, Options::empty());
    let mut content = String::new();
    html::push_html(&mut content, parser);

    (data, content)
}

// TODO: generate template
fn generate_template(_data: Data, content: String) -> String {
    content
}

fn get_html_file_path(path: &PathBuf) -> String {
    let filename = path.file_name().unwrap_or(OsStr::new("")).to_str().unwrap();

    format!("../blog/public/{}", str::replace(filename, "md", "html"))
}

fn save(file_path: String, html: String) -> std::io::Result<()> {
    println!("Saving to {}", file_path);

    fs::create_dir_all("../blog/public/")?;
    fs::write(file_path, html.as_bytes())?;

    Ok(())
}
