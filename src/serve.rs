use crate::build::run as run_build;
use crate::Config;
use anyhow::{Context, Result};
use rouille::Response;

/// Runs a local server to navigate the blog
pub fn run(build: bool) -> Result<()> {
    if build {
        run_build().context("Failed to build the blog")?;
    }

    println!("Listening to localhost:1337...");
    let config = &Config::new().context("Failed to retrieve the configuration")?;
    let path = config.build_path.clone();

    rouille::start_server("localhost:1337", move |request| {
        let response = rouille::match_assets(request, &path);
        if response.is_success() {
            return response;
        }

        Response::redirect_302("/index.html")
    });
}
