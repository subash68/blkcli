use clap::{Parser, Subcommand, ValueEnum};
use hex;
use std::path::PathBuf;

use hash::derivation::Hash;

mod hash;

/// Blkcli is a collection of tools that are meant to be useful while building, testing, and
/// running block chain applications.
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
    /// Provide common crypto hashing functions.
    Hash {
        #[clap(value_enum)]
        algorithm: Algorithm,

        /// Provide a filename to read and hash
        #[clap(value_parser)]
        input: PathBuf,

        #[clap(short, long)]
        string: bool,
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
    ParseWallet {},
}

#[derive(ValueEnum, Debug, Clone)]
enum Algorithm {
    Md5,
    Sha1,
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Hash {
            algorithm,
            input,
            string,
        } => {
            if string {
                let input_str = input.to_str().unwrap();
                println!(
                    "{:}",
                    hex::encode(Hash::compute(algorithm, input_str.as_bytes()))
                );
            } else {
                let file_data = std::fs::read(input).expect("file not found");
                println!("{:}", hex::encode(Hash::compute(algorithm, &file_data)));
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
        Commands::Wallet {} => {
            println!("Wallet reference for Wallet")
        }
        Commands::ParseWallet {} => {
            println!("Parse wallet command ")
        }
    }
}
