#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_plain;

mod api;
#[cfg(feature = "with_network")]
mod client;
mod error;
mod types;

pub use api::Api;

pub use api::json_string;
pub use api::opt_uuid_simple;
pub use api::prelude::BinancePayWebHookRequest;
pub use api::prelude::BinancePayWebHookResponse;
pub use api::prelude::BizStatus;
pub use api::prelude::Certificate;
pub use api::prelude::CertificateRequest;
pub use api::prelude::CertificateResponse;
pub use api::prelude::CloseOrderRequest;
pub use api::prelude::CloseOrderResponse;
pub use api::prelude::CreateOrderRequest;
pub use api::prelude::CreateOrderResponse;
pub use api::prelude::Notification;
pub use api::prelude::OrderResult;
pub use api::prelude::PayerInfo;
pub use api::prelude::QueryOrderRequest;
pub use api::prelude::QueryOrderResponse;
pub use api::prelude::QueryOrderResult;
pub use api::prelude::ReturnCode;
pub use api::prelude::TransferFundRequest;
pub use api::prelude::TransferFundResponse;
pub use api::prelude::TransferResult;
pub use api::prelude::TransferStatus;
pub use api::prelude::TransferType;
pub use api::uuid_simple;
pub use client::ApiCred;
pub use client::Config;
pub use client::KeyClosure;
pub use client::MerchantId;
pub use error::common_error::CommonBusinessError;
pub use error::LibError;
pub use types::enums::StatusOrder;
pub use types::enums::StatusRequest;
pub use types::enums::TradeType;
pub use types::time::Time;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_serde_value_to_string() {
        assert_eq!("PAY_SUCCESS", BizStatus::PaySuccess.name());
        assert_eq!("PAY_CLOSED", BizStatus::PayClosed.name());

        assert_eq!("SUCCESS", ReturnCode::Success.name());
        assert_eq!("FAIL", ReturnCode::Fail.name());

        assert_eq!("APP", TradeType::App.name());
        assert_eq!("WEB", TradeType::Web.name());

        assert_eq!("SUCCESS", StatusRequest::Success.name());
        assert_eq!("FAIL", StatusRequest::Fail.name());

        assert_eq!("INITIAL", StatusOrder::Initial.name());
        assert_eq!("PENDING", StatusOrder::Pending.name());
        assert_eq!("PAID", StatusOrder::Paid.name());
        assert_eq!("CANCELED", StatusOrder::Canceled.name());
        assert_eq!("ERROR", StatusOrder::Error.name());
        assert_eq!("REFUNDING", StatusOrder::Refunding.name());
        assert_eq!("REFUNDED", StatusOrder::Refunded.name());
        assert_eq!("EXPIRED", StatusOrder::Expired.name());

        assert_eq!(
            BizStatus::PaySuccess,
            BizStatus::from_name("PAY_SUCCESS").unwrap()
        );
        assert_eq!(
            ReturnCode::Success,
            ReturnCode::from_name("SUCCESS").unwrap()
        );
        assert_eq!(TradeType::App, TradeType::from_name("APP").unwrap());
        assert_eq!(
            StatusRequest::Success,
            StatusRequest::from_name("SUCCESS").unwrap()
        );
        assert_eq!(
            StatusOrder::Expired,
            StatusOrder::from_name("EXPIRED").unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_serde_value_to_string_error() {
        assert_eq!(
            BizStatus::PaySuccess,
            BizStatus::from_name("PAY_success").unwrap()
        );
        assert_eq!(
            ReturnCode::Success,
            ReturnCode::from_name("success").unwrap()
        );
        assert_eq!(TradeType::App, TradeType::from_name("APPs").unwrap());
        assert_eq!(
            StatusRequest::Success,
            StatusRequest::from_name("success").unwrap()
        );
        assert_eq!(
            StatusOrder::Expired,
            StatusOrder::from_name("eXPIRED").unwrap()
        );
    }
}
