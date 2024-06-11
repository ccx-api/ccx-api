use crate::api::prime::prelude::*;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(test, derive(enum_iterator::Sequence))]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum ActivityUserActionKind {
    #[serde(rename = "OTHER_ACTION")]
    Other,
    #[serde(rename = "ACTION_APPROVE")]
    Approve,
    #[serde(rename = "ACTION_REJECT")]
    Reject,
    #[serde(rename = "ACTION_INITIATE")]
    Initiate,
    #[serde(rename = "ACTION_CANCEL")]
    Cancel,
}
#[cfg(feature = "db")]
forward_display_to_serde!(ActivityUserAction);
#[cfg(feature = "db")]
forward_from_str_to_serde!(ActivityUserAction);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_roundtrip() {
        for typ in enum_iterator::all::<ActivityUserActionKind>() {
            let s = serde_json::to_string(&typ).unwrap();
            let typ2: ActivityUserActionKind = serde_json::from_str(&s).unwrap();
            assert_eq!(typ, typ2);
        }
    }
}
