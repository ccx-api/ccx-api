extern crate core;

#[macro_use]
pub mod macros;

pub mod api;
#[cfg(feature = "with_network")]
pub mod client;
pub mod notification;
pub mod util;

#[cfg(feature = "with_network")]
pub use self::with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    pub use super::api::GatepayApi;
}
