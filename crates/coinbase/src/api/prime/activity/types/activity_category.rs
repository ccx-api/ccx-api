use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(test, derive(enum_iterator::Sequence))]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum ActivityCategory {
    #[serde(rename = "OTHER_ACTIVITY_CATEGORY")]
    Other,
    #[serde(rename = "ACTIVITY_CATEGORY_ORDER")]
    Order,
    #[serde(rename = "ACTIVITY_CATEGORY_TRANSACTION")]
    Transaction,
    #[serde(rename = "ACTIVITY_CATEGORY_ACCOUNT")]
    Account,
    #[serde(rename = "ACTIVITY_CATEGORY_ADMIN")]
    Admin,
    #[serde(rename = "ACTIVITY_CATEGORY_ALLOCATION")]
    Allocation,
}
#[cfg(feature = "db")]
forward_display_to_serde!(ActivityCategory);
#[cfg(feature = "db")]
forward_from_str_to_serde!(ActivityCategory);

impl ActivityCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActivityCategory::Other => "OTHER_ACTIVITY_CATEGORY",
            ActivityCategory::Order => "ACTIVITY_CATEGORY_ORDER",
            ActivityCategory::Transaction => "ACTIVITY_CATEGORY_TRANSACTION",
            ActivityCategory::Account => "ACTIVITY_CATEGORY_ACCOUNT",
            ActivityCategory::Admin => "ACTIVITY_CATEGORY_ADMIN",
            ActivityCategory::Allocation => "ACTIVITY_CATEGORY_ALLOCATION",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_roundtrip() {
        for item in enum_iterator::all::<ActivityCategory>() {
            let s = serde_json::to_string(&item).unwrap();
            let deserialized: ActivityCategory = serde_json::from_str(&s).unwrap();
            assert_eq!(item, deserialized);
        }
    }

    #[test]
    fn test_as_str() {
        for item in enum_iterator::all::<ActivityCategory>() {
            let s = serde_plain::to_string(&item).unwrap();
            assert_eq!(s, item.as_str());
        }
    }
}
