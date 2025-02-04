// #![allow(warnings)]

extern crate core;

pub mod api;
#[cfg(feature = "with_network")]
pub mod client;
pub mod error;
pub mod util;
#[cfg(feature = "with_network")]
pub mod websocket;

#[cfg(feature = "with_network")]
pub use self::with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    pub use super::api::GateApi;
}
