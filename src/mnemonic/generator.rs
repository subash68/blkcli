// Include packages what will contain word list
use crate::mnemonic::{chinese_traditional, english, french, japanese};
use clap::ValueEnum;
use rand::rngs::OsRng;
use rand::Rng;
// pub mod english;

// Add enum for language
#[derive(ValueEnum, Debug, Clone)]
pub enum Language {
    English,
    French,
    //Italian,
    Japanese,
    //Korean,
    //Spanish,
    //Czech,
    ChineseTraditional,
    //ChineseSimplified,
}

pub struct Generate {}

impl Generate {
    pub fn words(size: usize, language: Language) -> String {
        let mut mnemonic_str: Vec<String> = Vec::new();
        let rnd_bits = Self::generate_rnd_bits();

        let mut word_list = vec![String::from("")];

        match language {
            Language::English => {
                let word_list = english::convert_to_list();

                for index in 0..size {
                    let sel_ind = rnd_bits[index] as usize;
                    mnemonic_str.push(word_list[sel_ind].to_string());
                }
            }
            Language::French => {
                let word_list = french::convert_to_list();
                for index in 0..size {
                    let sel_ind = rnd_bits[index] as usize;
                    mnemonic_str.push(String::from(word_list[sel_ind]));
                }
            }
            Language::Japanese => {
                let word_list = japanese::convert_to_list();
                for index in 0..size {
                    let sel_ind = rnd_bits[index] as usize;
                    mnemonic_str.push(String::from(word_list[sel_ind]));
                }
            }
            Language::ChineseTraditional => {
                let word_list = chinese_traditional::convert_to_list();
            }
        }
        for index in 0..size {
            let sel_ind = rnd_bits[index] as usize;
            mnemonic_str.push(String::from(word_list[sel_ind]));
        }
        mnemonic_str.join(" ")
    }

    // Provide size in bytes: 16 bytes - 128 bits, 32 bytes - 256 bits, 64 bytes - 512 bytes
    fn generate_rnd_bits() -> Vec<u64> {
        let mut rng = OsRng;
        let mut random_bytes = [0u64; 32];
        rng.fill(&mut random_bytes);

        let mut rnd_num = Vec::new();

        for byte in random_bytes {
            rnd_num.push(byte % 2048);
        }

        rnd_num
    }
}
