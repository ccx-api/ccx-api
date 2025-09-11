use std::ops;

use rand::Rng;
use rand::rng;

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub struct Nonce {
    nonce: String,
}

impl Nonce {
    pub fn random() -> Self {
        let mut rng = rng();
        let nonce = (0..16)
            .map(|_| {
                let idx = rng.random_range(0..ALPHABET.len());
                ALPHABET[idx] as char
            })
            .collect();
        Self::new(nonce)
    }

    pub fn new(nonce: String) -> Self {
        Self { nonce }
    }
}

impl From<Nonce> for String {
    fn from(nonce: Nonce) -> Self {
        nonce.nonce
    }
}

impl ops::Deref for Nonce {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.nonce
    }
}

impl AsRef<str> for Nonce {
    fn as_ref(&self) -> &str {
        self
    }
}
