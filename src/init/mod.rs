use anyhow::{anyhow, bail, Context, Result};
use console::style;
use rust_embed::RustEmbed;
use std::fs;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "src/init/static"]
#[exclude = ".DS_Store"]
struct Asset;

/// Initialise a new Orion project
pub fn run(path: &str) -> Result<()> {
    println!(
        "{}",
        style(format!("Initialize a new Orion project at {}", &path)).cyan()
    );

    if Path::new(path).exists() {
        bail!("The path at {} already exists", path.to_string());
    }

    create_directories(path)?;

    Asset::iter().try_for_each(|file| {
        let file_path = format!("{path}/{file}");
        println!("{}", style(format!("Creating {}...", &file_path)).dim());

        fs::write(
            &file_path,
            Asset::get(&file)
                .ok_or_else(|| anyhow!("The embeded asset was not found at path: {}", &file))?
                .data,
        )
        .with_context(|| format!("Failed to write the file at path {}", &file_path))
    })
}

fn create_directories(path: &str) -> Result<()> {
    let paths = vec![
        path.to_string(),
        format!("{path}/posts"),
        format!("{path}/static/images"),
        format!("{path}/themes/orion"),
        format!("{path}/themes/orion/assets"),
    ];

    paths.iter().try_for_each(|path| {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create the directory at path {path}"))
    })
}
