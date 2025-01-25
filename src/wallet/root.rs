use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

pub fn generate_seed() -> Vec<u8> {
    let password = b"password";
    let salt = b"salt";
    let iterations = 100_000;

    let mut key1 = [0u8; 64];

    pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut key1);

    key1.to_vec()
}
