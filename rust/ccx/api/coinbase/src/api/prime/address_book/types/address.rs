use crate::api::prime::prelude::*;

/// An entry in the address book, representing a cryptocurrency address and associated metadata.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PortfolioAddressBookEntry {
    /// UUID identifying this address book entry.
    pub id: Uuid,
    /// Currency symbol.
    pub currency_symbol: Atom,
    /// Name for this address book entry.
    pub name: String,
    /// Cryptocurrency address.
    pub address: String,
    /// Memo or destination tag for currencies which support them.
    pub account_identifier: String,
    /// Name of the account identifier. For instance: "Destination Tag".
    pub account_identifier_name: String,
    /// State of this address book entry.
    pub state: Atom,
    /// Link to a blockchain explorer.
    pub explorer_link: String,
    /// When this entry was last used for a transaction (optional).
    pub last_used_at: Option<DtCoinbasePrime>,
    /// When this entry was added to the address book.
    pub added_at: DtCoinbasePrime,
    /// Information about who added this entry.
    pub added_by: AddressBookEntryAddedBy,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AddressBookEntryAddedBy {
    pub id: String,
    pub name: String,
    pub avatar_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_deserialize_entry_doc() {
    //     let json = r#"{
    //         "id": "string",
    //         "currency_symbol": "string",
    //         "name": "string",
    //         "address": "string",
    //         "account_identifier": "string",
    //         "account_identifier_name": "string",
    //         "state": "string",
    //         "explorer_link": "string",
    //         "last_used_at": "string",
    //         "added_at": "string",
    //         "added_by": {
    //             "id": "string",
    //             "name": "string",
    //             "avatar_url": "string"
    //         }
    //     }"#;
    //     let sample = AddressBookEntry {
    //         id: "string".to_string(),
    //         currency_symbol: "string".to_string(),
    //         name: "string".to_string(),
    //         address: "string".to_string(),
    //         account_identifier: "string".to_string(),
    //         account_identifier_name: "string".to_string(),
    //         state: "string".to_string(),
    //         explorer_link: "string".to_string(),
    //         last_used_at: "string".to_string(),
    //         added_at: "string".to_string(),
    //         added_by: AddressBookEntryAddedBy {
    //             id: "string".to_string(),
    //             name: "string".to_string(),
    //             avatar_url: "string".to_string(),
    //         },
    //     };
    //     let entry: AddressBookEntry = serde_json::from_str(json).unwrap();
    //     assert_eq!(entry, sample);
    // }

    #[test]
    fn test_deserialize_entry_live() {
        let json = r#"{
            "id": "87654321-0000-0000-0000-000000000000",
            "currency_symbol": "usdt",
            "name": "Treasury",
            "address": "0x00112233445566778899aabbccddeeff00112233",
            "account_identifier": "",
            "account_identifier_name": "",
            "state": "in_use",
            "explorer_link": "https://etherscan.io/address/0x00112233445566778899aabbccddeeff00112233",
            "last_used_at": null,
            "added_at": "2024-03-12T12:40:25.585258Z",
            "added_by": {
                "id": "12345678-0000-0000-0000-000000000000",
                "name": "Treasury Europe, LLC",
                "avatar_url": ""
            },
            "type": "ADDRESS_BOOK_TYPE_ADDRESS",
            "counterparty_id": ""
        }"#;
        let sample = PortfolioAddressBookEntry {
            id: Uuid::parse_str("87654321-0000-0000-0000-000000000000").unwrap(),
            currency_symbol: "usdt".into(),
            name: "Treasury".to_string(),
            address: "0x00112233445566778899aabbccddeeff00112233".to_string(),
            account_identifier: "".to_string(),
            account_identifier_name: "".to_string(),
            state: "in_use".into(),
            explorer_link:
                "https://etherscan.io/address/0x00112233445566778899aabbccddeeff00112233"
                    .to_string(),
            last_used_at: None,
            added_at: DtCoinbasePrime::parse_from_str("2024-03-12T12:40:25.585258Z").unwrap(),
            added_by: AddressBookEntryAddedBy {
                id: "12345678-0000-0000-0000-000000000000".to_string(),
                name: "Treasury Europe, LLC".to_string(),
                avatar_url: "".to_string(),
            },
        };
        let entry: PortfolioAddressBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry, sample);
    }
}
