use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {}

impl Config {
    pub fn new() -> Self {
        // TODO: unwrap
        let contents = fs::read_to_string("./config.toml").unwrap();

        // TODO: unwrap
        toml::from_str(&contents).unwrap()
    }
}
