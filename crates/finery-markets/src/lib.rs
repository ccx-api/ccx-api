#![allow(warnings)]
#![allow(dead_code)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_repr;

mod api;
mod client;
mod error;
pub mod types;

#[cfg(feature = "with_network")]
pub use self::with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    pub use ccx_api_lib::ApiCred;
    pub use ccx_api_lib::Proxy;

    pub use super::api::SpotApi;
    pub use super::client::Config;
    pub use super::client::FinerySigner;
    pub use super::client::SignResult;
}

pub use error::LibError;
pub use error::LibResult;

pub use ccx_api_lib;
