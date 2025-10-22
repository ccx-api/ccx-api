#![allow(warnings)]

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

pub use ccx_api_lib;
#[cfg(feature = "uuid1")]
pub use uuid1::Uuid;
#[cfg(feature = "uuid08")]
pub use uuid08::Uuid;

pub use self::error::*;
pub use self::util::*;
