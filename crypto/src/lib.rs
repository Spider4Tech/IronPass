use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use argon2::{self, Config, Variant, Version};

pub fn derive_key(master_password: &str, salt: &[u8]) -> Vec<u8> {
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 3,
        lanes: 4,
        ..Default::default()
    };
    argon2::hash_raw(master_password.as_bytes(), salt, &config).unwrap()
}

pub fn encrypt_password(password: &str, key: &[u8]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new(key.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    cipher.encrypt(&nonce, password.as_bytes()).unwrap()
}

pub fn decrypt_password(encrypted: &[u8], key: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let cipher = ChaCha20Poly1305::new(key.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let decrypted = cipher.decrypt(&nonce, encrypted)?;
    Ok(String::from_utf8(decrypted)?)
}