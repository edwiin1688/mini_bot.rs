use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::RngCore;

const NONCE_SIZE: usize = 12;

pub fn encrypt(plaintext: &str, key: &str) -> Result<String, String> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes".to_string());
    }

    let cipher = Aes256Gcm::new_from_slice(key.as_bytes())
        .map_err(|e| format!("Failed to create cipher: {}", e))?;

    let mut nonce_bytes = [0u8; NONCE_SIZE];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| format!("Encryption failed: {}", e))?;

    let mut result = nonce_bytes.to_vec();
    result.extend(ciphertext);

    Ok(STANDARD.encode(result))
}

pub fn decrypt(encrypted: &str, key: &str) -> Result<String, String> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes".to_string());
    }

    let data = STANDARD
        .decode(encrypted)
        .map_err(|e| format!("Base64 decode failed: {}", e))?;

    if data.len() < NONCE_SIZE {
        return Err("Invalid encrypted data".to_string());
    }

    let cipher = Aes256Gcm::new_from_slice(key.as_bytes())
        .map_err(|e| format!("Failed to create cipher: {}", e))?;

    let nonce = Nonce::from_slice(&data[..NONCE_SIZE]);
    let ciphertext = &data[NONCE_SIZE..];

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;

    String::from_utf8(plaintext).map_err(|e| format!("UTF-8 conversion failed: {}", e))
}

pub fn get_encryption_key() -> Option<String> {
    std::env::var("MINIBOT_CONFIG_KEY").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = "01234567890123456789012345678901";
        let plaintext = "secret_api_key_12345";

        let encrypted = encrypt(plaintext, key).unwrap();
        let decrypted = decrypt(&encrypted, key).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_wrong_key_fails() {
        let key1 = "01234567890123456789012345678901";
        let key2 = "abcdefghijklmnopqrstuvwxyz012345";
        let plaintext = "secret_api_key_12345";

        let encrypted = encrypt(plaintext, key1).unwrap();
        let result = decrypt(&encrypted, key2);

        assert!(result.is_err());
    }
}
