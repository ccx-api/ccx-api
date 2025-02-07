#[cfg(feature = "console_formatter")]
pub mod nice_num;
#[cfg(feature = "console_formatter")]
pub use console;

#[inline]
pub fn default<T: Default>() -> T {
    Default::default()
}
