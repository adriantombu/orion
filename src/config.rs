use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub site_name: String,
    pub description: String,
    pub theme: String,
    pub build_path: PathBuf,
    pub locale: String,
    pub seo: Seo,
    pub twitter: Twitter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Seo {
    pub robots: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Twitter {
    pub creator: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let path = "./config.toml";

        toml::from_str(
            &fs::read_to_string("./config.toml")
                .with_context(|| format!("Failed to read at path {}", &path))?,
        )
        .context("Failed to parse the config file to TOML")
    }
}
