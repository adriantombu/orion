use crate::config::Config;
use anyhow::{Context, Result};
use console::style;
use fs_extra::dir::{copy, CopyOptions};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Assets {}

impl Assets {
    pub fn copy(config: &Config) -> Result<()> {
        println!("{}", style("Copying static assets...").dim());

        let theme_from = &format!("./themes/{}/assets", config.theme);
        let theme_to = &config.build_path;
        copy(theme_from, theme_to, &CopyOptions::new()).with_context(|| {
            format!(
                "Failed to copy theme assets from {} to {:?}",
                theme_from, theme_to
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
