#![allow(unused_imports, dead_code)]

use std::str::FromStr;

#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::AsExpression;
#[cfg(feature = "with_diesel_1-4")]
use diesel_derives::FromSqlRow;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
pub enum TradeType {
    #[serde(rename = "WEB")]
    Web,
    #[serde(rename = "APP")]
    App,
}
derive_display_from_serialize!(TradeType);
derive_fromstr_from_deserialize!(TradeType);

impl TradeType {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
pub enum StatusRequest {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAIL")]
    Fail,
}
derive_display_from_serialize!(StatusRequest);
derive_fromstr_from_deserialize!(StatusRequest);

impl StatusRequest {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
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
derive_display_from_serialize!(StatusOrder);
derive_fromstr_from_deserialize!(StatusOrder);

impl StatusOrder {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[cfg(feature = "with_diesel_1-4")]
mod db_impl {
    use std::io::Write;

    use diesel::deserialize::FromSql;
    use diesel::serialize::ToSql;

    use super::StatusOrder;
    use super::StatusRequest;
    use super::TradeType;

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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
pub enum TerminalType {
    #[serde(rename = "APP")]
    App,
    #[serde(rename = "WEB")]
    Web,
    #[serde(rename = "WAP")]
    Wap,
    #[serde(rename = "MINI_PROGRAM")]
    MiniProgram,
    #[serde(rename = "OTHERS")]
    Others,
}
derive_display_from_serialize!(TerminalType);
derive_fromstr_from_deserialize!(TerminalType);

impl TerminalType {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
pub enum OsType {
    #[serde(rename = "IOS")]
    IOs,
    #[serde(rename = "ANDRIOD")]
    Andriod,
}
derive_display_from_serialize!(OsType);
derive_fromstr_from_deserialize!(OsType);

impl OsType {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
pub enum GoodsCategory {
    #[serde(rename = "0000")]
    _0000, //: Electronics & Computers
    #[serde(rename = "1000")]
    _1000, //: Books, Music & Movies
    #[serde(rename = "2000")]
    _2000, //: Home, Garden & Tools
    #[serde(rename = "3000")]
    _3000, //: Clothes, Shoes & Bags
    #[serde(rename = "4000")]
    _4000, //: Toys, Kids & Baby
    #[serde(rename = "5000")]
    _5000, //: Automotive & Accessories
    #[serde(rename = "6000")]
    _6000, //: Game & Recharge
    #[serde(rename = "7000")]
    _7000, //: Entertainament & Collection
    #[serde(rename = "8000")]
    _8000, //: Jewelry
    #[serde(rename = "9000")]
    _9000, //: Domestic service
    #[serde(rename = "A000")]
    _A000, //: Beauty care
    #[serde(rename = "B000")]
    _B000, //: Pharmacy
    #[serde(rename = "C000")]
    _C000, //: Sports & Outdoors
    #[serde(rename = "D000")]
    _D000, //: Food, Grocery & Health products
    #[serde(rename = "E000")]
    _E000, //: Pet supplies
    #[serde(rename = "F000")]
    _F000, //: Industry & Science
    #[serde(rename = "Z000")]
    #[default]
    _Z000, //: Others
}
derive_display_from_serialize!(GoodsCategory);
derive_fromstr_from_deserialize!(GoodsCategory);

impl GoodsCategory {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
pub enum AddressType {
    #[serde(rename = "01")]
    _01, //: office
    #[serde(rename = "02")]
    _02, //: home
    #[serde(rename = "03")]
    _03, //: public box
    #[serde(rename = "04")]
    _04, //: others
}
derive_display_from_serialize!(AddressType);
derive_fromstr_from_deserialize!(AddressType);

impl AddressType {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Default)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
pub enum GoodsType {
    #[serde(rename = "01")]
    #[default]
    _01, //: Tangible Goods
    #[serde(rename = "02")]
    _02, //: Virtual Goods
}
derive_display_from_serialize!(GoodsType);
derive_fromstr_from_deserialize!(GoodsType);

impl GoodsType {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
pub enum TransferType {
    #[serde(rename = "TO_MAIN")]
    ToMain,
    #[serde(rename = "TO_PAY")]
    ToPay,
}
derive_display_from_serialize!(TransferType);
derive_fromstr_from_deserialize!(TransferType);

impl TransferType {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "with_diesel_1-4", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "with_diesel_1-4", sql_type = "diesel::sql_types::Text")]
pub enum TransferStatus {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "PROCESS")]
    Process,
}
derive_display_from_serialize!(TransferStatus);
derive_fromstr_from_deserialize!(TransferStatus);

impl TransferStatus {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}
