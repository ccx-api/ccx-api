#![allow(unused_imports)]

use std::str::FromStr;

#[cfg(feature = "db")]
use diesel_derives::{AsExpression, FromSqlRow};

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum TradeType {
    #[serde(rename = "WEB")]
    Web,
    #[serde(rename = "APP")]
    App,
}
forward_display_to_serde!(TradeType);
forward_from_str_to_serde!(TradeType);

impl TradeType {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum StatusRequest {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAIL")]
    Fail,
}
forward_display_to_serde!(StatusRequest);
forward_from_str_to_serde!(StatusRequest);

impl StatusRequest {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum StatusOrder {
    #[serde(rename = "INITIAL")]
    Initial,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "PAID")]
    Paid,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "REFUNDING")]
    Refunding,
    #[serde(rename = "REFUNDED")]
    Refunded,
    #[serde(rename = "EXPIRED")]
    Expired,
}
forward_display_to_serde!(StatusOrder);
forward_from_str_to_serde!(StatusOrder);

impl StatusOrder {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[cfg(feature = "db")]
mod db_impl {
    use std::io::Write;

    use diesel::deserialize::FromSql;
    use diesel::serialize::ToSql;

    use super::{StatusOrder, StatusRequest, TradeType};

    impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for TradeType
    where
        DB: diesel::backend::Backend,
        str: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
    {
        fn to_sql<W: std::io::Write>(
            &self,
            out: &mut diesel::serialize::Output<W, DB>,
        ) -> diesel::serialize::Result {
            self.name().as_str().to_sql(out)
        }
    }

    impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for TradeType
    where
        DB: diesel::backend::Backend,
        String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
            let name = String::from_sql(bytes)?;
            Self::from_name(name.as_str()).ok_or_else(|| {
                format!("Unrecognized name {:?} for {}", name, stringify!($name)).into()
            })
        }
    }

    impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for StatusRequest
    where
        DB: diesel::backend::Backend,
        str: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
    {
        fn to_sql<W: std::io::Write>(
            &self,
            out: &mut diesel::serialize::Output<W, DB>,
        ) -> diesel::serialize::Result {
            self.name().as_str().to_sql(out)
        }
    }

    impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for StatusRequest
    where
        DB: diesel::backend::Backend,
        String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
            let name = String::from_sql(bytes)?;
            Self::from_name(name.as_str()).ok_or_else(|| {
                format!("Unrecognized name {:?} for {}", name, stringify!($name)).into()
            })
        }
    }

    impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for StatusOrder
    where
        DB: diesel::backend::Backend,
        str: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
    {
        fn to_sql<W: std::io::Write>(
            &self,
            out: &mut diesel::serialize::Output<W, DB>,
        ) -> diesel::serialize::Result {
            self.name().as_str().to_sql(out)
        }
    }

    impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for StatusOrder
    where
        DB: diesel::backend::Backend,
        String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
            let name = String::from_sql(bytes)?;
            Self::from_name(name.as_str()).ok_or_else(|| {
                format!("Unrecognized name {:?} for {}", name, stringify!($name)).into()
            })
        }
    }
}
