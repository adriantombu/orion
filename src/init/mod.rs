use anyhow::{anyhow, bail, Context, Result};
use console::style;
use rust_embed::RustEmbed;
use std::fs;
use std::fs::File;
use std::io::Write;
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
        bail!("The path at {} does not exists", path.to_string());
    }

    fs::create_dir_all(path)
        .with_context(|| format!("Failed to create the directory at path {}", path))?;
    fs::create_dir_all(format!("{}/posts", path))
        .with_context(|| format!("Failed to create the directory at path {}/posts", path))?;
    fs::create_dir_all(format!("{}/static/images", path)).with_context(|| {
        format!(
            "Failed to create the directory at path {}/static/images",
            path
        )
    })?;
    fs::create_dir_all(format!("{}/themes/orion", path)).with_context(|| {
        format!(
            "Failed to create the directory at path {}/themes/orion",
            path
        )
    })?;

    Asset::iter().try_for_each(|file| {
        let file_path = format!("{}/{}", path, file);
        println!("{}", style(format!("Creating {}...", &file_path)).dim());

        let asset = Asset::get(&file)
            .ok_or_else(|| anyhow!("The embeded asset was not found at path: {}", &file))?;
        let mut f = File::create(Path::new(&file_path))
            .with_context(|| format!("Failed to create the file at path {}", &file_path))?;

        f.write_all(&asset.data)
            .with_context(|| format!("Failed to write the file at path {}", &file_path))
    })
}
