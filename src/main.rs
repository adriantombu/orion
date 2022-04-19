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
        Commands::Article { name } => {
            println!("Creating a new article {}", name);
        }
        Commands::Build => {
            println!("Building the blog");
        }
        Commands::Init { path } => {
            println!("Initialize a new Orion project in {}", path);
        }
        Commands::Serve => {
            println!("Serving html blog");
        }
    }
}
