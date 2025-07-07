///  int64;
///  Internal size unit. One unit is equal to 0.00000001 (1e-8) fraction of asset unit: for example, 1 USD is 100000000 size units
pub type Size = i64;

///  unsigned int64;
///  Internal price unit. One unit is equal to 0.00000001 (1e-8) fraction of instrument's balance currency unit
pub type Price = u64;

///  unsigned int64;
///  UTC unix timestamp with milliseconds
pub type Timestamp = u64;

///  unsigned int64;
///  Deal or settlement id
pub type DealId = u64;

///  unsigned int64;
///  Order or settlement order id
pub type OrderId = u64;

///  unsigned int64;
///  Order id as defined by a user
pub type ClientOrderId = u64;

///  unsigned int32;
///  User id
pub type ClientId = u32;

///  unsigned int16;
///      0 = bid
///      1 = ask
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SideByRepr {
    /// bid
    Bid = 0,
    /// ask
    Ask = 1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SideByName {
    /// bid
    #[serde(rename = "bid")]
    Bid,
    /// ask
    #[serde(rename = "ask")]
    Ask,
}

///  unsigned int16;
///  Order cancel reason
///      0 = in place or filled
///      1 = by client
///      2 = as non-book order
///      3 = by self-trade prevention
///      4 = by cancel-on-disconnect)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum CancelReason {
    /// in place or filled
    InPlaceOrFilled = 0,
    /// by client
    ByClient = 1,
    /// as non-book order
    AsNonBookOrder = 2,
    /// by self-trade prevention
    BySelfTradePrevention = 3,
    /// by cancel-on-disconnect
    ByCancelOnDisconnect = 4,
}

impl CancelReason {
    pub fn as_code(&self) -> u16 {
        *self as u16
    }

    pub fn description(&self) -> &str {
        match self {
            Self::InPlaceOrFilled => "In place or filled",
            Self::ByClient => "Canceled by client",
            Self::AsNonBookOrder => "Canceled as non-book order",
            Self::BySelfTradePrevention => "Canceled by self-trade prevention",
            Self::ByCancelOnDisconnect => "Canceled by cancel-on-disconnect",
        }
    }
}

impl Default for CancelReason {
    fn default() -> Self {
        Self::InPlaceOrFilled
    }
}

///  unsigned int16;
///  Value depends on context
#[allow(dead_code)]
pub type Flags = u16;

///  unsigned int16;
///  See  section for list of error codes
#[allow(dead_code)]
pub type ErrorCode = u16;

///  unsigned int16;
///  Order Type
///     0 - limit
///     1 - post only
///     2 - limit IOC
///     3 - limit FOK
///     4 - market IOC
///     5 - market FOK
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum OrderTypeByRepr {
    /// limit
    Limit = 0,
    /// post only
    PostOnly = 1,
    /// limit IOC
    LimitIOC = 2,
    /// limit FOK
    LimitFOK = 3,
    /// market IOC
    MarketIOC = 4,
    /// market FOK
    MarketFOK = 5,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OrderTypeByName {
    /// limit
    #[serde(rename = "limit")]
    Limit,
    /// post only
    #[serde(rename = "postOnly")]
    PostOnly,
    /// limit IOC
    #[serde(rename = "limitIOC")]
    LimitIOC,
    /// limit FOK
    #[serde(rename = "limitFOK")]
    LimitFOK,
    /// market IOC
    #[serde(rename = "marketIOC")]
    MarketIOC,
    /// market FOK
    #[serde(rename = "marketFOK")]
    MarketFOK,
}

///  unsigned int16;
///  If order was created by size or by volume
///     0 - by size
///     1 - by volume
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum OrderCreateType {
    /// by size
    BySize = 0,
    /// by volume
    ByVolume = 1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SettlementFlags {
    /// No flags
    NoFlags = 0,
    /// Fee paid by recipient
    PaidByRecipient = 1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientType {
    Maker,
    Taker,
    Master,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientStatus {
    /// Connection request has been accepted.
    Connected,
    /// You have sent an invitation to connect. But it has not been accepted yet.
    InviteSent,
    /// A counterparty has sent an invitation to connect.
    InvitePending,
}
