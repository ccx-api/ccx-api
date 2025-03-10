use rust_decimal::Decimal;
use serde::Serialize;
use serde::{Deserialize, Deserializer, Serializer, de};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kline {
    pub open_time: u64,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    pub close_time: u64,
    pub quote_asset_volume: Decimal,
}

impl Serialize for Kline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(8))?;
        seq.serialize_element(&self.open_time)?;
        seq.serialize_element(&self.open)?;
        seq.serialize_element(&self.high)?;
        seq.serialize_element(&self.low)?;
        seq.serialize_element(&self.close)?;
        seq.serialize_element(&self.volume)?;
        seq.serialize_element(&self.close_time)?;
        seq.serialize_element(&self.quote_asset_volume)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Kline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::SeqAccess;
        use serde::de::Visitor;

        struct KlineVisitor;

        impl<'de> Visitor<'de> for KlineVisitor {
            type Value = Kline;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of 8 elements")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Kline, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let open_time = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let open = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let high = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let low = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let close = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let volume = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(5, &self))?;
                let close_time = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(6, &self))?;
                let quote_asset_volume = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(7, &self))?;
                Ok(Kline {
                    open_time,
                    open,
                    high,
                    low,
                    close,
                    volume,
                    close_time,
                    quote_asset_volume,
                })
            }
        }

        deserializer.deserialize_seq(KlineVisitor)
    }
}

#[derive(
    Serialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::IntoStaticStr,
    strum::EnumString
)]
pub enum KlineInterval {
    Min1,
    Min5,
    Min15,
    Min30,
    Min60,
    Hour4,
    Hour8,
    Day1,
    Week1,
    Month1
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn it_deserializes_doc_example() {
        let json = r#"
          [
            1640804880000,
            "47482.36",
            "47482.36",
            "47416.57",
            "47436.1",
            "3.550717",
            1640804940000,
            "168387.3"
        ]
        "#;

        let expected = Kline {
            open_time: 1640804880000,
            open: dec!(47482.36),
            high: dec!(47482.36),
            low: dec!(47416.57),
            close: dec!(47436.1),
            volume: dec!(3.550717),
            close_time: 1640804940000,
            quote_asset_volume: dec!(168387.3),
        };
        let actual = serde_json::from_str::<Kline>(json).unwrap();
        assert_eq!(actual, expected);
    }
}
