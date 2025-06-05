use bon::Builder;

#[derive(Builder)]
#[non_exhaustive]
pub struct BitGoSignerPayload<'a> {
    pub path: &'a str,
    pub body: &'a str,
}

pub trait BitGoSigner: Send {
    fn token(&self) -> &str;
}

impl<T> BitGoSigner for &T
where
    T: BitGoSigner + Sync,
{
    fn token(&self) -> &str {
        (*self).token()
    }
}
