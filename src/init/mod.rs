use rust_embed::RustEmbed;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

#[derive(RustEmbed)]
#[folder = "src/init/static"]
#[exclude = ".DS_Store"]
struct Asset;

/// Initialise a new Orion project
pub fn run(path: &str) -> Result<(), InitError> {
    println!("Initialize a new Orion project in {}", path);

    if Path::new(path).exists() {
        return Err(InitError::PathExists(path.to_string()));
    }

    fs::create_dir_all(path)?;
    fs::create_dir_all(format!("{}/posts", path))?;
    fs::create_dir_all(format!("{}/static/images", path))?;
    fs::create_dir_all(format!("{}/themes/orion", path))?;

    Asset::iter().try_for_each(|file| {
        let file_path = format!("{}/{}", path, file);
        println!("Creating {}", file_path);

        let asset =
            Asset::get(&file).ok_or_else(|| InitError::AssetNotFound(file_path.to_string()))?;
        let mut f = File::create(Path::new(file_path.as_str()))?;
        Ok(f.write_all(&asset.data)?)
    })
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error("{0}")]
    StdIo(#[from] std::io::Error),

    #[error("Path \"{0}\" already exists, please chose another one")]
    PathExists(String),

    #[error("Asset at path \"{0}\" was not found")]
    AssetNotFound(String),
}
