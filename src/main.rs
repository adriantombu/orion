mod article;
mod build;
mod init;
mod serve;

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

    /// Builds and serve the blog
    Serve,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Article { name } => article::run(&name),
        Commands::Build => build::run(),
        Commands::Init { path } => init::run(&path),
        Commands::Serve => serve::run(),
    }
}
