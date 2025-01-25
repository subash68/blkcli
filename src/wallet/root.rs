use base58::ToBase58;
use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2_hmac;
use sha2::{Digest, Sha256, Sha512};

type HmacSha512 = Hmac<Sha512>;

pub struct WalletGenerator {}

impl WalletGenerator {
    pub fn generate_xprv(seed: &[u8]) -> String {
        // Step 1: HMAC-SHA512 with "Bitcoin seed" as key
        let mut mac =
            HmacSha512::new_from_slice(b"Bitcoin seed").expect("HMAC can take a key of any size");
        mac.update(seed);
        let result = mac.finalize().into_bytes();

        // Split the result into master private key and chain code
        let master_private_key = &result[0..32];
        let chain_code = &result[32..64];

        // Step 2: Construct xprv binary structure
        let mut xprv = vec![];
        xprv.extend(&[0x04, 0x88, 0xAD, 0xE4]); // Version
        xprv.push(0x00); // Depth
        xprv.extend(&[0x00, 0x00, 0x00, 0x00]); // Parent fingerprint
        xprv.extend(&[0x00, 0x00, 0x00, 0x00]); // Child index
        xprv.extend(chain_code); // Chain code
        xprv.push(0x00); // Leading byte for private key
        xprv.extend(master_private_key); // Master private key

        // Step 3: Add checksum
        let checksum = Sha256::digest(Sha256::digest(&xprv));
        xprv.extend(&checksum[0..4]);

        // Step 4: Encode in Base58Check
        xprv.to_base58()
    }

    pub fn generate_seed(mnemonic: &str) -> Vec<u8> {
        let password = format!("{:?}", mnemonic);
        let salt = "mnemonic".to_string();
        let iterations = 2048; // number of rounds

        let mut key1 = [0u8; 64];

        pbkdf2_hmac::<Sha256>(password.as_bytes(), salt.as_bytes(), iterations, &mut key1);

        key1.to_vec()
    }
}
