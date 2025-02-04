use derive_more::Deref;
use serde::de::Deserializer;
use serde::de::Visitor;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EitherOrderId {
    Bitstamp(i64),
    Client(Uuid),
}

impl std::fmt::Display for EitherOrderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EitherOrderId::Bitstamp(order_id) => write!(f, "{order_id}"),
            EitherOrderId::Client(order_id) => write!(f, "client:{order_id}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Deref, PartialEq, Eq)]
pub struct OrderId(i64);

impl<'de> Deserialize<'de> for OrderId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrderIdVisitor;

        impl<'de> Visitor<'de> for OrderIdVisitor {
            type Value = OrderId;

            fn expecting(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                fmt.write_str("integer or string")
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(OrderId(v as i64))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(OrderId(v as i64))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(OrderId(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(OrderId(i64::try_from(v).map_err(|e| {
                    serde::de::Error::custom(format!("overflow, expected i64: {}", e))
                })?))
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match val.parse::<i64>() {
                    Ok(val) => self.visit_i64(val),
                    Err(_) => Err(E::custom("failed to parse integer")),
                }
            }
        }

        deserializer.deserialize_any(OrderIdVisitor)
    }
}
