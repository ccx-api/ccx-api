extern crate core;

pub mod api;
#[cfg(feature = "with_network")]
pub mod client;
pub mod error;
pub mod proto;
pub mod util;

#[cfg(all(feature = "uuid08", feature = "uuid1"))]
compile_error!("Cannot use both `uuid08` and `uuid1` features simultaneously.");
#[cfg(not(any(feature = "uuid08", feature = "uuid1")))]
compile_error!("At least one `uuid` feature must be enabled: `uuid08` or `uuid1`.");

#[cfg(feature = "uuid08")]
pub use uuid08::Uuid;
#[cfg(feature = "uuid1")]
pub use uuid1::Uuid;

// pub use self::client::CoinbaseSigner;
// pub use self::client::Nonce;
// pub use self::client::SignResult;
pub use self::error::*;
pub use self::proto::*;
pub use self::util::*;
#[cfg(feature = "with_network")]
pub use self::with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    // pub use super::api::spot::SpotApi;
    // pub use super::api::prime::PrimeApi;
}
