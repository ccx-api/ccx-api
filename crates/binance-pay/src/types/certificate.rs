#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Certificate {
    #[serde(rename = "certSerial")]
    pub cert_serial: String,
    #[serde(rename = "certPublic")]
    pub cert_public: String,
}
