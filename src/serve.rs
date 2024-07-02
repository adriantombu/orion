use crate::build::run as run_build;
use crate::Config;
use anyhow::{Context, Result};
use console::style;
use rouille::Response;

/// Runs a local server to navigate the blog
pub fn run() -> Result<()> {
    run_build().context("Failed to build the blog")?;

    println!("{}", style("Listening to http://localhost:1337...").green());

    let config = Config::new().context("Failed to retrieve the configuration")?;
    rouille::start_server("localhost:1337", move |request| {
        let response = rouille::match_assets(request, &config.build_path);
        if response.is_success() {
            return response;
        }

        Response::redirect_302("/index.html")
    });
}
