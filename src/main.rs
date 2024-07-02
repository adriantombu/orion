extern crate core;

mod build;
mod config;
mod init;
mod post;
mod serve;

use crate::config::Config;
use anyhow::Context;

const HELP: &str = "\
A simple static blog generator

Usage: orion-ssg <COMMAND>

Commands:
  init --path my-blog                   Initialise a new Orion project
  post --slug my-amazing-title --draft  Create a new empty post
  build                                 Builds the blog to html
  serve                                 Runs a local server to navigate the blog

Options:
  -h, --help     Print help
  -v, --version  Print version";

#[derive(Debug)]
struct AppArgs {
    /// The slug of the post
    slug: Option<String>,

    /// Set to true to create the new post as a draft (it won't be published)
    draft: Option<String>,

    /// Path of the new project
    path: Option<String>,
    command: String,
}

fn main() {
    match parse_args() {
        Ok(args) => {
            match args.command.as_str() {
                "post" => post::run(
                    &args.slug.expect("--slug is required"),
                    args.draft.unwrap_or_default() == "--draft",
                )
                .context("Failed to create a new post"),

                "build" => build::run().context("Failed to build the blog"),

                "init" => init::run(&args.path.expect("--path is required"))
                    .context("Failed to initialize a new project"),

                "serve" => serve::run().context("Failed to serve the blog locally"),

                _ => {
                    print!("{HELP}");
                    std::process::exit(0);
                }
            }
            .expect("TODO: panic message");
        }
        Err(e) => {
            eprintln!("Error: {e}.");
            std::process::exit(1);
        }
    }
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.clone().finish().is_empty() || pargs.contains(["-h", "--help"]) {
        print!("{HELP}");
        std::process::exit(0);
    }

    if pargs.contains(["-v", "--version"]) {
        print!("orion-ssg v{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    let args = AppArgs {
        slug: pargs.opt_value_from_str("--slug")?,
        draft: pargs.opt_value_from_str("--draft")?,
        path: pargs.opt_value_from_str("--path")?,
        command: pargs.free_from_str()?,
    };

    Ok(args)
}
