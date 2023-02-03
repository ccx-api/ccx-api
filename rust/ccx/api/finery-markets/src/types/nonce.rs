use crate::types::Time;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct Nonce(pub u64);

impl Nonce {
    pub fn new(v: impl Into<u64>) -> Self {
        Nonce(v.into())
    }
}

impl From<Time> for Nonce {
    fn from(t: Time) -> Self {
        Self::new(t.0)
    }
}

pub struct NonceSeq {
    last: u64,
}

impl NonceSeq {
    pub fn new(last: u64) -> Self {
        NonceSeq { last }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Nonce {
        let mut now = Time::now().0;
        if now <= self.last {
            now = self.last + 1;
        }
        self.last = now;
        Nonce(self.last)
    }
}

impl From<Time> for NonceSeq {
    fn from(t: Time) -> Self {
        Self::new(t.0)
    }
}
