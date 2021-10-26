use std::fmt;

use serde::de;

use super::super::prelude::*;

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct TxIds<'a>(pub &'a [&'a str]);

impl<'a> fmt::Display for TxIds<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut div = "";
        for id in self.0 {
            write!(f, "{}{}", div, id)?;
            div = ",";
        }
        Ok(())
    }
}

impl<'a> Serialize for TxIds<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
