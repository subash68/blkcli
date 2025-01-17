use crate::mnemonic::generator::Language;
use std::fs::File;
use std::io::{self, BufRead};

pub fn read_bip39_words(language: Language) -> io::Result<Vec<String>> {
    let file = match language {
        Language::English => File::open("bip39_wordlist/english.txt"),
        Language::French =>  File::open("bip39_wordlist/french.txt"),
        Language::Italian => File::open("bip39_wordlist/italian.txt"),
        Language::Japanese =>File::open("bip39_wordlist/japanese.txt"),
        Language::Korean => File::open("bip39_wordlist/korean.txt"),
        Language::Spanish => File::open("bip39_wordlist/spanish.txt"),
        Language::Czech => File::open("bip39_wordlist/czech.txt"),
        Language::ChineseTraditional => File::open("bip39_wordlist/chinese_traditional.txt"),
        Language::ChineseSimplified => File::open("bip39_wordlist/chinese_simplified.txt"),
    };

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
