use std::fmt;

use serde::de;

use super::super::prelude::*;

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct OrderFlags {
    /// Post-only order (available when ordertype = limit).
    pub post: bool,
    /// Prefer fee in base currency (default if selling).
    pub fcib: bool,
    /// Prefer fee in quote currency (default if buying, mutually exclusive with fcib).
    pub fciq: bool,
    /// Disable market price protection for market orders.
    pub nompp: bool,
}

impl fmt::Display for OrderFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flags = &[
            ("post", self.post),
            ("fcib", self.fcib),
            ("fciq", self.fciq),
            ("nompp", self.nompp),
        ];
        let mut div = "";
        for &(flag, is_set) in flags {
            if is_set {
                write!(f, "{}{}", div, flag)?;
                div = ",";
            }
        }
        Ok(())
    }
}

impl Serialize for OrderFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO serialize via a tiny buffer.
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for OrderFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FlagsVisitor;

        impl<'de> de::Visitor<'de> for FlagsVisitor {
            type Value = OrderFlags;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string with comma separated list of identifiers")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                const FLAGS: &[&str] = &["post", "fcib", "fciq", "nompp"];

                let mut v = OrderFlags::default();
                for s in value.split(',') {
                    match s {
                        "post" => v.post = true,
                        "fcib" => v.fcib = true,
                        "fciq" => v.fciq = true,
                        "nompp" => v.nompp = true,
                        _ => Err(de::Error::unknown_variant(value, FLAGS))?,
                    }
                }

                Ok(v)
            }
        }

        deserializer.deserialize_str(FlagsVisitor)
    }
}
