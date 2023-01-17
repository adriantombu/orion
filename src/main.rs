extern crate core;

mod build;
mod config;
mod init;
mod post;
mod serve;

use crate::config::Config;
use anyhow::Context;
use clap::{Parser, Subcommand};
use console::style;

/// A simple static blog generator
#[derive(Debug, Parser)]
#[clap(
    author = "Adrian Tombu <adrian@otso.fr>",
    version,
    about = "A simple static blog generator",
    long_about = "Write your post in Markdown and build them into a static HTML website"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new empty post
    #[clap(arg_required_else_help = true)]
    Post {
        /// The slug of the post
        slug: String,

        /// Set to true to create the new post as a draft (it won't be published)
        #[arg(short, long, default_value_t = false)]
        draft: bool,
    },

    /// Builds the blog to html
    Build,

    /// Initialise a new Orion project
    #[clap(arg_required_else_help = true)]
    Init {
        /// Path of the new project
        path: String,
    },

    /// Runs a local server to navigate the blog
    Serve {
        /// Set to true to generate the blog before launching the server
        #[arg(short, long, default_value_t = false)]
        build: bool,
    },
}

fn main() {
    let args = Cli::parse();

    let res = match args.command {
        Commands::Post { slug, draft } => {
            post::run(&slug, draft).context("Failed to create a new post")
        }
        Commands::Build => build::run().context("Failed to build the blog"),
        Commands::Init { path } => init::run(&path).context("Failed to initialize a new project"),
        Commands::Serve { build } => serve::run(build).context("Failed to serve the blog locally"),
    };

    if let Err(err) = res {
        eprintln!("{:?}", style(err).red());
        std::process::exit(1);
    }
}
