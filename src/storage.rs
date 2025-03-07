use crate::crypto::Encryptor;
use rand::{Rng, distributions::Alphanumeric};
use sled::Db;

pub struct Storage {
    db: Db,
    encryptor: Encryptor,
}

impl Storage {
    pub fn new(db_path: &str, key: &[u8]) -> Self {
        Self {
            db: sled::open(db_path).expect("Failed to open database"),
            encryptor: Encryptor::new(key),
        }
    }

    pub fn save_note(&self, note: &str) -> String {
        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric) // ✅ Make sure this line uses Alphanumeric
            .take(8)
            .map(char::from)
            .collect();

        let encrypted = self.encryptor.encrypt(note);

        match self.db.insert(id.as_bytes(), encrypted.as_bytes()) {
            Ok(_) => println!("✅ Note saved with ID: {}", id),
            Err(e) => eprintln!("❌ Failed to save note: {}", e),
        }

        self.db.flush().unwrap(); // Ensure data is written to disk

        id
    }

    pub fn get_and_delete(&self, id: &str) -> Option<String> {
        println!("🔍 Looking for note ID: {}", id);

        if let Ok(Some(data)) = self.db.remove(id.as_bytes()) {
            self.db.flush().unwrap(); // Ensure deletion is saved
            let encrypted_text = String::from_utf8(data.to_vec()).ok()?;
            println!("✅ Found and deleting note (encrypted): {}", encrypted_text);

            let decrypted_text = self.encryptor.decrypt(&encrypted_text);
            if let Some(plain_text) = &decrypted_text {
                println!("🔓 Successfully decrypted note: {}", plain_text);
            } else {
                println!("❌ Decryption failed!");
            }
            decrypted_text
        } else {
            println!("❌ Note NOT found!");
            None
        }
    }
}
