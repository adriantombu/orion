use crate::build::run as run_build;
use crate::Config;
use rouille::Response;

/// Runs a local server to navigate the blog
pub fn run(build: bool) {
    if build {
        run_build().expect("Error while building")
    }

    println!("Listening to localhost:1337...");
    let config = &Config::new().expect("Unable to retrieve configuration");
    let path = config.build_path.clone();

    rouille::start_server("localhost:1337", move |request| {
        let response = rouille::match_assets(request, &path);
        if response.is_success() {
            return response;
        }

        Response::redirect_302("/index.html")
    });
}
