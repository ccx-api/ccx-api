extern crate core;

pub mod api;
#[cfg(feature = "with_network")]
pub mod client;
pub mod error;
pub mod proto;
pub mod util;

// pub use client::BitstampSigner;
// pub use client::Nonce;
// pub use client::SignResult;
pub use error::*;
pub use proto::*;
pub use util::*;
#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    // pub use super::api::spot::SpotApi;
    // pub use super::api::prime::PrimeApi;
}
