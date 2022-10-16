extern crate core;

mod article;
mod build;
mod config;
mod init;
mod parser;
mod serve;

use crate::config::Config;
use clap::{Parser, Subcommand};

/// A simple static blog generator
#[derive(Debug, Parser)]
#[clap(
    author = "Adrian Tombu <adrian@otso.fr>",
    version,
    about = "A simple static blog generator",
    long_about = "Write your article in Markdown and build them into a static HTML website"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new empty article
    #[clap(arg_required_else_help = true)]
    Article {
        /// The filename of the article
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

fn main() {
    let args = Cli::parse();
    let config = Config::new().expect("Unable to retrieve configuration");

    match args.command {
        Commands::Article { name } => article::run(&name),
        Commands::Build => build::run(&config).expect("Error while building"),
        Commands::Init { path } => init::run(&path),
        Commands::Serve { build } => serve::run(build, &config),
    }
}
