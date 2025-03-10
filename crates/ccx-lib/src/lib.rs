#[cfg(feature = "console_formatter")]
pub mod nice_num;
#[cfg(feature = "console_formatter")]
pub use console;
mod error;
pub mod rate_limiter;
#[cfg(feature = "websocket")]
pub mod websocket;

pub use error::*;

#[inline]
pub fn default<T: Default>() -> T {
    Default::default()
}
