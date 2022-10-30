extern crate core;

mod build;
mod config;
mod init;
mod post;
mod serve;

use crate::config::Config;
use clap::{Parser, Subcommand};

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
        /// The filename of the post
        name: String,
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

// TODO: use anyhow instead of thiserror
fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Post { name } => post::run(&name),
        Commands::Build => build::run().expect("Error while building"),
        Commands::Init { path } => {
            init::run(&path).expect("Error while initializing a new Orion project")
        }
        Commands::Serve { build } => serve::run(build),
    }
}
