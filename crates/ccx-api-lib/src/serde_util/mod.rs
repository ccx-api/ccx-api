pub mod f64_arbitrary_precision;
/// (De)serialize empty string as [None]
///
/// Unlike `serde_with::NoneAsEmptyString`, it uses `Serialize` & `Deserialize`
/// instead of [FromStr](std::str::FromStr) & [Display](core::fmt::Display)
pub mod none_as_empty_str;

use serde::Deserialize;
use serde::Deserializer;

pub fn is_false(val: &bool) -> bool {
    !*val
}

pub fn default_on_null<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
}
