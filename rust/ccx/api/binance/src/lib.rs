pub use rust_decimal::Decimal;
pub use string_cache::DefaultAtom as Atom;

pub mod api;
#[cfg(feature = "with_network")]
pub mod client;
pub mod error;
pub mod proto;
pub mod util;

pub use self::error::*;
pub use self::proto::*;

#[cfg(feature = "with_network")]
pub use self::with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    pub use super::api::spot::SpotApi;
    pub use super::api::um::UmApi;
}