use clap::{Parser, Subcommand};
use std::path::PathBuf;

// Define modules here based on what Commands
//

/// A documentation for utility that we are going to build
///
/// For more information type sub command with -- help
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short = 'n', long, env)]
    name_some: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Hash any give file or string
    /// some options should be described here
    Hash {
        /// This is help for options in the sub command
        #[clap(short, long)]
        title: Option<String>,
    },
    /// mnenomic handle
    /// Generates random mnenomic based on give parameter
    Mnenomic {
        /// Defines numnber of words to be generated (Default: 15)
        #[clap(short, long)]
        words: Option<u8>,

        /// Defines which language mnemonic should be generated (Default: English)
        #[clap(short, long)]
        language: Option<String>,
    },
    Wallet {},
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Hash { ref title } => {
            if let Some(value) = title {
                println!("Detected value from hash command {}", value);
            }
        }
        Commands::Mnenomic {
            ref words,
            ref language,
        } => {
            let words_length = words.unwrap_or(15);
            let lang = language.as_deref().unwrap_or("English");

            println!(
                "Detected value from mnenomic with {}, {}",
                words_length, lang
            );
        }
    }
    dbg!(args);
}
