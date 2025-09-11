#[cfg(feature = "with_awc")]
pub mod awc;
#[cfg(feature = "with_reqwest")]
pub mod reqwest;

// Re-export common types based on active feature
#[cfg(all(feature = "with_awc", not(feature = "with_reqwest")))]
pub use awc::*;
#[cfg(all(feature = "with_reqwest", not(feature = "with_awc")))]
pub use reqwest::*; // Default to awc for backward compatibility
