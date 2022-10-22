use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

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
    pub fn new() -> Result<Self, ConfigError> {
        Ok(toml::from_str(&fs::read_to_string("./config.toml")?)?)
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Unable to read config file: {0}")]
    StdIoError(#[from] std::io::Error),

    #[error("Unable to parse config file: {0}")]
    TomlDeserializeError(#[from] toml::de::Error),
}
