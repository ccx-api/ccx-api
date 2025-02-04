#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Pair {
    pub base: String,
    pub quote: String,
}

impl Pair {
    pub fn new<B: Into<String>, Q: Into<String>>(base: B, quote: Q) -> Self {
        Self {
            base: base.into(),
            quote: quote.into(),
        }
    }
}

impl serde::Serialize for Pair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!("{}-{}", self.base, self.quote);
        serializer.serialize_str(&s)
    }
}

impl<'de> serde::Deserialize<'de> for Pair {
    fn deserialize<D>(deserializer: D) -> Result<Pair, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut items = s.split('-');
        let base = items
            .next()
            .ok_or_else(|| serde::de::Error::custom("Unknown pair base."))?;
        let quote = items
            .next()
            .ok_or_else(|| serde::de::Error::custom("Unknown pair quote."))?;
        Ok(Pair {
            base: String::from(base),
            quote: String::from(quote),
        })
    }
}
