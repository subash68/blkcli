// Include packages what will contain word list
use crate::mnemonic::english;
use clap::ValueEnum;
use rand::rngs::OsRng;
use rand::Rng;
// pub mod english;

// Add enum for language
#[derive(ValueEnum, Debug, Clone)]
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
    pub fn words(size: usize, language: Language) -> String {
        let mut mnemonic_str: Vec<String> = Vec::new();

        //TODO: Based on select language read the word list

        let word_list = english::convert_to_list();
        let random_bytes = Self::generate_rnd_bits();

        for index in (0..size) {
            let sel_ind = random_bytes[index] as usize;
            mnemonic_str.push(word_list[sel_ind].to_string());
        }
        mnemonic_str.join(" ")
    }

    // Provide size in bytes: 16 bytes - 128 bits, 32 bytes - 256 bits, 64 bytes - 512 bytes
    fn generate_rnd_bits() -> Vec<u64> {
        let mut rng = OsRng;
        let mut random_bytes = [0u64; 24];
        rng.fill(&mut random_bytes);

        let mut rnd_num = Vec::new();

        for byte in random_bytes {
            rnd_num.push(byte % 2048);
        }

        rnd_num
    }
}
