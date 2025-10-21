use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(test, derive(enum_iterator::Sequence))]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
pub enum ActivityStatus {
    #[serde(rename = "OTHER_ACTIVITY_STATUS")]
    Other,
    #[serde(rename = "ACTIVITY_STATUS_CANCELLED")]
    Cancelled,
    #[serde(rename = "ACTIVITY_STATUS_PROCESSING")]
    Processing,
    #[serde(rename = "ACTIVITY_STATUS_COMPLETED")]
    Completed,
    #[serde(rename = "ACTIVITY_STATUS_EXPIRED")]
    Expired,
    #[serde(rename = "ACTIVITY_STATUS_REJECTED")]
    Rejected,
    #[serde(rename = "ACTIVITY_STATUS_FAILED")]
    Failed,
}
#[cfg(feature = "with_diesel_1-4")]
forward_display_to_serde!(ActivityStatus);
#[cfg(feature = "with_diesel_1-4")]
forward_from_str_to_serde!(ActivityStatus);

impl ActivityStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActivityStatus::Other => "OTHER_ACTIVITY_STATUS",
            ActivityStatus::Cancelled => "ACTIVITY_STATUS_CANCELLED",
            ActivityStatus::Processing => "ACTIVITY_STATUS_PROCESSING",
            ActivityStatus::Completed => "ACTIVITY_STATUS_COMPLETED",
            ActivityStatus::Expired => "ACTIVITY_STATUS_EXPIRED",
            ActivityStatus::Rejected => "ACTIVITY_STATUS_REJECTED",
            ActivityStatus::Failed => "ACTIVITY_STATUS_FAILED",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_roundtrip() {
        for item in enum_iterator::all::<ActivityStatus>() {
            let s = serde_json::to_string(&item).unwrap();
            let deserialize: ActivityStatus = serde_json::from_str(&s).unwrap();
            assert_eq!(item, deserialize);
        }
    }

    #[test]
    fn test_as_str() {
        for item in enum_iterator::all::<ActivityStatus>() {
            let s = serde_plain::to_string(&item).unwrap();
            assert_eq!(s, item.as_str());
        }
    }
}
