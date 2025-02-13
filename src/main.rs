use clap::{Parser, Subcommand};
use std::path::PathBuf;

use hash::derivation::{Algorithm, Hash};
use mnemonic::generator::{Generate, Language};
use wallet::root::WalletGenerator;

mod hash;
mod mnemonic;
mod wallet;

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
enum SubCommands {
    Create {
        #[clap(short, long)]
        root_only: bool,

        #[clap(short, long)]
        words: Option<u8>,

        #[clap(short, long)]
        language: Option<Language>,
    },
    Inspect {},
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
    Mnemonic {
        /// Defines numnber of words to be generated (Default: 24)
        #[clap(short, long)]
        words: Option<u8>,

        /// Defines which language mnemonic should be generated (Default: English)
        #[clap(value_enum)]
        language: Option<Language>,
    },
    Wallet {
        #[clap(subcommand)]
        cmd: SubCommands,
    },
    ParseWallet {},
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
        Commands::Mnemonic {
            ref words,
            language,
        } => {
            let words_length = words.unwrap_or(15);
            let lang = language.unwrap_or(Language::English);

            println!("Generating {} word mnemonic for {:?}", words_length, lang);
            println!("{}", Generate::words(words_length.into(), lang));
        }
        Commands::Wallet { cmd } => {
            match cmd {
                SubCommands::Create {
                    ref words,
                    language,
                    root_only,
                } => {
                    // TODO: Make this default word length into constant
                    let words_length = words.unwrap_or(15);
                    let lang = language.unwrap_or(Language::English);

                    println!(
                        "Creating wallet for {}, {:?}, {}",
                        words_length, lang, root_only
                    );

                    let words = Generate::words(words_length.into(), lang);
                    let seed = WalletGenerator::generate_seed(&words);
                    let xprv_root_key = WalletGenerator::generate_xprv(&seed);

                    println!("mnemonic: {:?}", words);
                    println!("seed: {:?}", hex::encode(seed));
                    println!("root key: {:?}", xprv_root_key);
                }
                SubCommands::Inspect {} => {}
            }
        }
        Commands::ParseWallet {} => {
            println!("Parse wallet command ")
        }
    }
}
