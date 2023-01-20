use crate::config::Config;
use anyhow::{Context, Result};
use console::style;
use fs_extra::dir::{copy, CopyOptions};
use serde::Serialize;
use std::fs;

#[derive(Debug, Serialize, Clone)]
pub struct Assets {}

impl Assets {
    pub fn copy(config: &Config) -> Result<()> {
        println!("{}", style("Copying static assets...").dim());

        let favicon_from = &format!("./themes/{}/favicon.png", config.theme);
        let favicon_to = &format!("{}/favicon.png", config.build_path.display());
        fs::copy(favicon_from, favicon_to).with_context(|| {
            format!(
                "Failed to copy favicon from {} to {}",
                favicon_from, favicon_to
            )
        })?;

        let style_from = &format!("./themes/{}/style.css", config.theme);
        let style_to = &format!("{}/style.css", config.build_path.display());
        fs::copy(style_from, style_to).with_context(|| {
            format!(
                "Failed to copy stylesheet from {} to {}",
                style_from, style_to
            )
        })?;

        let images_from = "./static/images";
        let images_to = &config.build_path;
        copy(images_from, images_to, &CopyOptions::new()).with_context(|| {
            format!(
                "Failed to copy images from {} to {:?}",
                images_from, images_to
            )
        })?;

        Ok(())
    }
}
