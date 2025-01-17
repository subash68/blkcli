use crate::mnemonic::wordlist;
use clap::ValueEnum;
use rand::rngs::OsRng;
use rand::Rng;

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
        let rnd_bits = Self::generate_rnd_bits();

        let bip39_words: Result<Vec<String>, std::io::Error> = wordlist::read_bip39_words(language);
        match bip39_words {
            Ok(words) => {
                // check if the bits generated is less than required
                for index in 0..size {
                    let sel_ind = rnd_bits[index] as usize;
                    if sel_ind < words.len() {
                        mnemonic_str.push(String::from(&words[sel_ind]));
                    } else {
                        eprintln!("Index {} is out of bound for word vectors", sel_ind);
                    }
                }
            },
            Err(e) => eprintln!("Unable to convert lines: {}", e),
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
