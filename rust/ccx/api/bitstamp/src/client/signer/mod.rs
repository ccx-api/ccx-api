mod nonce;
mod sign;

pub use nonce::*;
pub use sign::*;

pub trait BitstampSigner: Sync + Send {
    fn sign_data<'a, 'b: 'a, 'c: 'b>(
        &'c self,
        nonce: Nonce,
        timestamp: u64,
        http_method: &'b str,
        http_host: &'b str,
        http_path: &'b str,
        http_query: &'b str,
        content_type: &'b str,
        version: &'b str,
        body: &'b str,
    ) -> SignResult<'a>;

    fn api_key(&self) -> &str;
}
