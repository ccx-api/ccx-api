use derive_more::Deref;
use derive_more::From;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deref, From)]
pub struct Nonce(Uuid);

impl Nonce {
    /// Creates a new nonce based on UUID v4.
    pub fn new() -> Nonce {
        Nonce(Uuid::new_v4())
    }
}

impl Default for Nonce {
    fn default() -> Self {
        Self::new()
    }
}
