mod frontmatter;
mod tagger;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mdbook-tagging", about = "Generate tag index pages for mdbook")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run as mdbook preprocessor (reads JSON from stdin)
    Preprocess,
    /// Check if a renderer is supported
    Supports {
        /// Renderer name to check
        renderer: String,
    },
    /// Generate tag index pages directly
    Generate {
        /// Path to mdbook root directory
        #[arg(default_value = ".")]
        book_dir: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Preprocess => {
            tagger::run_preprocess()?;
        }
        Commands::Supports { renderer } => {
            // Support all renderers
            if renderer == "html" || renderer == "markdown" {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        }
        Commands::Generate { book_dir } => {
            tagger::generate_tags(&book_dir)?;
        }
    }

    Ok(())
}
