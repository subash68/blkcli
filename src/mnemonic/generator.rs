// Include packages what will contain word list

use rand::rngs::OsRng;
use rand::Rng;

// pub mod english;

// Add enum for language
pub enum Language {
    English,
    French,
    Italian,
    Japanese,
    Korean,
    Spanish,
    Czech,
    ChineseTraditional,
    ChineseSimplified,
}

pub struct Generate {}

impl Generate {
    pub fn words(size: usize, language: Language) -> Vec<String> {
        let mut mnemonic_str: Vec<String> = Vec::new();

        mnemonic_str.push(String::from("some"));

        mnemonic_str
    }
}

// Provide size in bytes: 16 bytes - 128 bits, 32 bytes - 256 bits, 64 bytes - 512 bytes
pub fn generate_rnd_bits() {
    let mut rng = OsRng;
    let mut random_bytes = [0u64; 24];
    rng.fill(&mut random_bytes);

    println!("{}", random_bytes.len());
    for byte in random_bytes {
        println!("{}", (byte % 2048));
    }
}
