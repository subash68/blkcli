use clap::ValueEnum;
use md5;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

#[derive(ValueEnum, Debug, Clone)]
pub enum Algorithm {
    Md5,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha521_224,
    Sha521_256,
}

pub struct Hash {}

impl Hash {
    pub fn compute(algorithm: Algorithm, input: &[u8]) -> Vec<u8> {
        match algorithm {
            Algorithm::Md5 => md5::compute(input).to_vec(),
            Algorithm::Sha256 => Sha256::digest(input).to_vec(),
            Algorithm::Sha224 => Sha224::digest(input).to_vec(),
            Algorithm::Sha384 => Sha384::digest(input).to_vec(),
            Algorithm::Sha512 => Sha512::digest(input).to_vec(),
            Algorithm::Sha521_224 => Sha512_224::digest(input).to_vec(),
            Algorithm::Sha521_256 => Sha512_256::digest(input).to_vec(),
        }
    }
}
