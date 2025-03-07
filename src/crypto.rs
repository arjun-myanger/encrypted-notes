use base64::{decode, encode};
use ring::aead::{AES_256_GCM, Aad, LessSafeKey, NONCE_LEN, Nonce, UnboundKey};
use ring::rand::{SecureRandom, SystemRandom}; // For generating random nonces

pub struct Encryptor {
    key: LessSafeKey,
}

impl Encryptor {
    pub fn new(secret_key: &[u8]) -> Self {
        println!("🔑 Encryption Key: {:?}", secret_key); // Debug: Print the key
        let unbound_key = UnboundKey::new(&AES_256_GCM, secret_key).expect("Invalid Key");
        Self {
            key: LessSafeKey::new(unbound_key),
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let rng = SystemRandom::new();
        let mut nonce_bytes = [0u8; NONCE_LEN];
        rng.fill(&mut nonce_bytes).expect("Nonce generation failed"); // ✅ Generate random nonce
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let mut data = plaintext.as_bytes().to_vec();

        println!("🛠 Encrypting message: {}", plaintext);
        println!("🔢 Generated Nonce: {:?}", nonce_bytes); // Debug: Show the nonce

        self.key
            .seal_in_place_append_tag(nonce, Aad::empty(), &mut data)
            .expect("Encryption failed");

        let mut combined = Vec::with_capacity(NONCE_LEN + data.len()); // Ensure capacity
        combined.extend_from_slice(&nonce_bytes); // ✅ Store nonce first
        combined.extend_from_slice(&data); // ✅ Store encrypted data after

        let encrypted_text = encode(&combined);
        println!("🔐 Encrypted (Base64): {}", encrypted_text); // Debug: Show encrypted output

        encrypted_text
    }

    pub fn decrypt(&self, encrypted: &str) -> Option<String> {
        let data = decode(encrypted).ok()?; // ✅ Decode base64
        if data.len() < NONCE_LEN {
            println!("❌ Decryption failed: Data too short.");
            return None;
        }

        let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);

        println!("🔓 Trying to decrypt: {}", encrypted);
        println!("🔢 Extracted Nonce: {:?}", nonce_bytes); // Debug: Check the extracted nonce

        let nonce =
            Nonce::assume_unique_for_key(nonce_bytes.try_into().expect("Nonce conversion failed"));
        let mut decrypted_data = ciphertext.to_vec();

        let result = self
            .key
            .open_in_place(nonce, Aad::empty(), &mut decrypted_data);

        match result {
            Ok(plaintext) => {
                let decrypted_text = String::from_utf8(plaintext.to_vec()).expect("Invalid UTF-8");
                println!("✅ Successfully decrypted: {}", decrypted_text);
                Some(decrypted_text)
            }
            Err(_) => {
                println!("❌ Decryption failed: Incorrect nonce or corrupted data.");
                None
            }
        }
    }
}
