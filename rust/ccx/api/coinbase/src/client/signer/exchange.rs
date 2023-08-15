use core::fmt;
use std::future::Future;
use std::pin::Pin;
use std::str::from_utf8_unchecked;

use ccx_api_lib::ExchangeApiCred;

use crate::CoinbaseResult;

pub type ExchangeSignResult<'a> = Pin<Box<dyn Future<Output = CoinbaseResult<String>> + Send + 'a>>;

pub trait CoinbaseExchangeSigner: Sync + Send {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        timestamp: u32,
        method: &'b str,
        url_path: &'b str,
        json_payload: &'b str,
    ) -> ExchangeSignResult<'a>;

    fn api_key(&self) -> &str;

    fn api_passphrase(&self) -> &str;
}

impl CoinbaseExchangeSigner for ExchangeApiCred {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        timestamp: u32,
        method: &'b str,
        url_path: &'b str,
        json_payload: &'b str,
    ) -> ExchangeSignResult<'a> {
        Box::pin(async move {
            Ok(sign(
                &self.secret,
                timestamp,
                method,
                url_path,
                json_payload,
            ))
        })
    }

    fn api_key(&self) -> &str {
        self.key.as_str()
    }

    fn api_passphrase(&self) -> &str {
        self.passphrase.as_str()
    }
}

fn sign(secret: &[u8], timestamp: u32, method: &str, url_path: &str, json_payload: &str) -> String {
    use base64::engine::general_purpose;
    use base64::Engine as _;
    use hmac::Hmac;
    use hmac::Mac;
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_from_slice(secret).expect("HMAC can take key of any size");

    mac.update(ArrStr::from_u32(timestamp).as_ref());
    mac.update(method.as_bytes());
    mac.update(url_path.as_bytes());
    mac.update(json_payload.as_bytes());

    let payload = mac.finalize().into_bytes();
    general_purpose::STANDARD.encode(payload)
}

#[derive(Clone, Copy)]
pub struct ArrStr<const N: usize> {
    len: usize,
    buf: [u8; N],
}

impl<const N: usize> ArrStr<N> {
    pub fn new(v: impl fmt::Display) -> Option<ArrStr<N>> {
        use std::io::Write;

        let mut buf = [0; N];
        let len = {
            let mut cursor = std::io::Cursor::new(buf.as_mut());
            // Expected to be successful always.
            write!(&mut cursor, "{v}").ok()?;
            cursor.position() as usize
        };
        Some(ArrStr { len, buf })
    }
}

impl ArrStr<0> {
    // fn from_u64(v: u64) -> ArrStr<20> {
    //     // 20 - the length of u64::max_value.
    //     ArrStr::new(v).unwrap()
    // }

    fn from_u32(v: u32) -> ArrStr<10> {
        // 10 - the length of u32::max_value.
        ArrStr::new(v).unwrap()
    }
}

impl<const N: usize> AsRef<[u8]> for ArrStr<N> {
    fn as_ref(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

impl<const N: usize> AsRef<str> for ArrStr<N> {
    fn as_ref(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.buf[..self.len]) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn secret() -> Vec<u8> {
        let secret = "jlQQwYehIhgWMZsbhDuSUDH/f7ATwkxME+fB4GxZKgvUvSg/L2rel7VBTotJ81dsMTohjK2zCjHN4cWHckRXWg==";
        ExchangeApiCred::decode_secret(Some(secret))
    }

    #[test]
    fn test_decode() {
        assert_eq!(hex::encode(secret()), "8e5410c187a1221816319b1b843b925031ff7fb013c24c4c13e7c1e06c592a0bd4bd283f2f6ade97b5414e8b49f3576c313a218cadb30a31cde1c5877244575a")
    }

    #[test]
    fn test_sign() {
        let timestamp = 1616783157;
        let method = "GET";
        let url_path = "/orders?status=open&product_id=BTC-USD";
        let json_payload = "";
        let secret = secret();
        let expected_signature = "VEV9fDqcepoyVHJ1lXhAuwFCyfR4RQRfy6TyHBEnooI=";

        let signature = sign(&secret, timestamp, method, url_path, json_payload);
        assert_eq!(signature, expected_signature);
    }
}
