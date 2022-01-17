use std::str::from_utf8_unchecked;

use chrono::Utc;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct Nonce(u64);

#[derive(Clone, Copy)]
pub struct NonceDecimal {
    len: usize,
    // 20 - the length of u64::max_value.
    buf: [u8; 20],
}

#[derive(Serialize)]
pub struct NonceWrapper<T>
where
    T: Serialize,
{
    pub nonce: Nonce,
    #[serde(flatten)]
    pub payload: T,
}

pub struct NonceSeq {
    last: u64,
}

impl Nonce {
    pub fn new(v: impl Into<u64>) -> Self {
        Nonce(v.into())
    }

    pub(crate) fn value(&self) -> u64 {
        self.0
    }

    pub fn decimal(self) -> NonceDecimal {
        use std::io::Write;

        let mut buf = [0; 20];
        let len = {
            let mut cursor = std::io::Cursor::new(buf.as_mut());
            // Expected to be successful always.
            let _ = write!(&mut cursor, "{}", self.0);
            cursor.position() as usize
        };
        NonceDecimal { len, buf }
    }

    pub fn wrap<T>(self, payload: T) -> NonceWrapper<T>
    where
        T: Serialize,
    {
        let nonce = self;
        NonceWrapper { nonce, payload }
    }
}

impl AsRef<[u8]> for NonceDecimal {
    fn as_ref(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

impl AsRef<str> for NonceDecimal {
    fn as_ref(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.buf[..self.len]) }
    }
}

impl NonceSeq {
    pub fn new() -> Self {
        let last = 0;
        NonceSeq { last }
    }

    /// Creates a new nonce based on current timestamp
    /// and checks that new nonce is greater than previous one.
    pub fn ts_next(&mut self) -> Nonce {
        let mut now = Utc::now().timestamp_millis() as u64;
        if now <= self.last {
            now = self.last + 1;
        }
        self.last = now;
        Nonce(now)
    }
}
