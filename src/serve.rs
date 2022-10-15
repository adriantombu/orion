use crate::build::run as run_build;
use crate::Config;
use rouille::Response;

/// Runs a local server to navigate the blog
pub fn run(build: bool, config: &Config) {
    if build {
        run_build(config).expect("Error while building")
    }

    println!("Listening to localhost:1337...");
    rouille::start_server("localhost:1337", move |request| {
        let response = rouille::match_assets(request, "public");
        if response.is_success() {
            return response;
        }

        // TODO: proper 404 page
        Response::html(
            "404 error. Try <a href=\"/README.md\"`>README.md</a> or \
                        <a href=\"/src/lib.rs\">src/lib.rs</a> for example.",
        )
        .with_status_code(404)
    });
}
