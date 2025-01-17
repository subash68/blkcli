use crate::mnemonic::generator::Language;
use std::fs::File;
use std::io::{self, BufRead};

pub fn read_bip39_words(language: Language) -> io::Result<Vec<String>> {
    let file_name = match language {
        Language::English => "bip39_wordlist/english.txt",
        Language::French => "bip39_wordlist/french.txt",
        Language::Italian => "bip39_wordlist/italian.txt",
        Language::Japanese => "bip39_wordlist/japanese.txt",
        Language::Korean => "bip39_wordlist/korean.txt",
        Language::Spanish => "bip39_wordlist/spanish.txt",
        Language::Czech => "bip39_wordlist/czech.txt",
        Language::ChineseTraditional => "bip39_wordlist/chinese_traditional.txt",
        Language::ChineseSimplified => "bip39_wordlist/chinese_simplified.txt",
    };

    let file = File::open(file_name);
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            eprint!("Error while reading file: {}", e);
            return Err(e);
        }
    };

    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(content) =>  lines.push(content),
            Err(e) => eprintln!("Error while reading file: {}", e),
        }
    }
    Ok(lines)
}
