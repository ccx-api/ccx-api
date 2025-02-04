use std::fmt;

use thiserror::Error;

#[derive(Clone, Error)]
pub enum CommonBusinessError {
    UnknowError,
    InvalidRequest,
    InvalidSignature,
    InvalidTimestamp,
    InvalidApiKeyOrIp,
    BadApiKeyFmt,
    MandatoryParamEmptyOrMalformed,
    InvalidParamWrongLength,
    InvalidParamWrongValue,
    InvalidParamIllegalChar,
    InvalidRequestTooLarge,
    InvalidMerchantTradeNo,
    OrderNotFound,
    InvalidAccountStatus,
}

impl fmt::Display for CommonBusinessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CommonBusinessError {{ code: {}; reason: {} }}",
            self.code(),
            self.reason()
        )
    }
}

impl fmt::Debug for CommonBusinessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommonBusinessError")
            .field("code", &self.code())
            .field("reason", &self.reason())
            .finish()
    }
}

impl CommonBusinessError {
    pub fn code(&self) -> i32 {
        match self {
            Self::UnknowError => 400000,
            Self::InvalidRequest => 400001,
            Self::InvalidSignature => 400002,
            Self::InvalidTimestamp => 400003,
            Self::InvalidApiKeyOrIp => 400004,
            Self::BadApiKeyFmt => 400005,
            Self::MandatoryParamEmptyOrMalformed => 400100,
            Self::InvalidParamWrongLength => 400101,
            Self::InvalidParamWrongValue => 400102,
            Self::InvalidParamIllegalChar => 400103,
            Self::InvalidRequestTooLarge => 400104,
            Self::InvalidMerchantTradeNo => 400201,
            Self::OrderNotFound => 400202,
            Self::InvalidAccountStatus => 400203,
        }
    }

    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            400000 => Some(Self::UnknowError),
            400001 => Some(Self::InvalidRequest),
            400002 => Some(Self::InvalidSignature),
            400003 => Some(Self::InvalidTimestamp),
            400004 => Some(Self::InvalidApiKeyOrIp),
            400005 => Some(Self::BadApiKeyFmt),
            400100 => Some(Self::MandatoryParamEmptyOrMalformed),
            400101 => Some(Self::InvalidParamWrongLength),
            400102 => Some(Self::InvalidParamWrongValue),
            400103 => Some(Self::InvalidParamIllegalChar),
            400104 => Some(Self::InvalidRequestTooLarge),
            400201 => Some(Self::InvalidMerchantTradeNo),
            400202 => Some(Self::OrderNotFound),
            400203 => Some(Self::InvalidAccountStatus),
            _ => None,
        }
    }

    pub fn reason(&self) -> &str {
        match self {
            Self::UnknowError => "An unknown error occurred while processing the request.",
            Self::InvalidRequest => {
                "Parameter format is wrong or parameter transferring doesn't follow the rules."
            }
            Self::InvalidSignature => "Incorrect signature result",
            Self::InvalidTimestamp => "Timestamp for this request is outside of the time window.",
            Self::InvalidApiKeyOrIp => "Api key not found or invalid.",
            Self::BadApiKeyFmt => "Api key format invalid.",
            Self::MandatoryParamEmptyOrMalformed => {
                "A parameter was missing/empty/null, or malformed."
            }
            Self::InvalidParamWrongLength => {
                "A parameter was not valid, was empty/null, or too long/short, or wrong format."
            }
            Self::InvalidParamWrongValue => {
                "A parameter was not valid, the value is out of range.	"
            }
            Self::InvalidParamIllegalChar => {
                "A parameter was not valid, contains illegal characters"
            }
            Self::InvalidRequestTooLarge => "Invalid request, content length too large",
            Self::InvalidMerchantTradeNo => "merchantTradeNo is invalid or duplicated",
            Self::OrderNotFound => "Order not found.",
            Self::InvalidAccountStatus => {
                "Not support for this account, please check account status."
            }
        }
    }

    pub fn solution(&self) -> &str {
        match self {
            Self::UnknowError                       => "Try again later",
            Self::InvalidRequest                    => "Please check whether the parameters are correct.",
            Self::InvalidSignature                  => "Check whether the signature parameter and method comply with signature algorithm requirements.",
            Self::InvalidTimestamp                  => "Sync server clock",
            Self::InvalidApiKeyOrIp                 => "Check api key",
            Self::BadApiKeyFmt                      => "Check api key.",
            Self::MandatoryParamEmptyOrMalformed    => "",
            Self::InvalidParamWrongLength           => "",
            Self::InvalidParamWrongValue            => "",
            Self::InvalidParamIllegalChar           => "",
            Self::InvalidRequestTooLarge            => "",
            Self::InvalidMerchantTradeNo            => "",
            Self::OrderNotFound                     => "",
            Self::InvalidAccountStatus              => "",
        }
    }
}
