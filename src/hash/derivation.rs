use crate::Algorithm;
use md5;

pub struct Hash {}

impl Hash {
    pub fn compute(algorithm: Algorithm, input: &[u8]) -> Vec<u8> {
        match algorithm {
            Algorithm::Md5 => md5::compute(input).to_vec(),
            Algorithm::Sha1 => vec![0],
        }
    }
}
