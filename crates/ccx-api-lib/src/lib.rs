pub use rust_decimal;
pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;
pub use string_cache;
pub use string_cache::DefaultAtom as Atom;

#[cfg(any(feature = "with_awc", feature = "with_reqwest"))]
mod client;
#[cfg(feature = "with_awc")]
mod connector;
mod cred;
mod env;
#[cfg(feature = "with_env_logger")]
pub mod env_logger_util;
mod error;
mod proxy;
mod rate_limiter;
mod seq;
pub mod serde_util;

// Re-export awc types at root level for backward compatibility
#[cfg(feature = "with_awc")]
pub use self::client::awc::{
    Client, ClientRequest, ClientResponse, Method, PayloadError, SendRequestError, StatusCode,
    make_client,
};
#[cfg(any(feature = "with_awc", feature = "with_reqwest"))]
pub use self::client::*;
#[cfg(feature = "with_awc")]
pub use self::connector::*;
pub use self::cred::*;
pub use self::env::*;
pub use self::error::*;
pub use self::proxy::*;
pub use self::seq::*;
