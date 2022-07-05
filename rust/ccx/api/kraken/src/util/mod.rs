mod order_book;

pub use self::order_book::*;

/// Converts Kraken's asset codes into ccxt-compatible asset codes.
pub fn universal_asset_code(asset_id: &str) -> &str {
    if !asset_id.is_ascii() {
        return asset_id;
    }
    let asset_id =
        if asset_id.len() > 3 && matches!(&asset_id[0..1], "X" | "Z") && !asset_id.contains('.') {
            &asset_id[1..]
        } else {
            asset_id
        };
    match () {
        () if asset_id.eq_ignore_ascii_case("XBT") => "BTC",
        () if asset_id.eq_ignore_ascii_case("BCC") => "BCH",
        () if asset_id.eq_ignore_ascii_case("DRK") => "DASH",
        () if asset_id.eq_ignore_ascii_case("BCHABC") => "BCH",
        () if asset_id.eq_ignore_ascii_case("BCHSV") => "BSV",
        () => asset_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert() {
        assert_eq!("ADA", universal_asset_code("ADA"));
        assert_eq!("BTC", universal_asset_code("XXBT"));
        assert_eq!("ETH", universal_asset_code("XETH"));
        assert_eq!("SOL", universal_asset_code("SOL"));
        assert_eq!("XLM", universal_asset_code("XXLM"));
        assert_eq!("XMR", universal_asset_code("XXMR"));
        assert_eq!("XRP", universal_asset_code("XXRP"));
        assert_eq!("ZEC", universal_asset_code("XZEC"));

        assert_eq!("AUD", universal_asset_code("ZAUD"));
        assert_eq!("CAD", universal_asset_code("ZCAD"));
        assert_eq!("CHF", universal_asset_code("CHF"));
        assert_eq!("EUR", universal_asset_code("ZEUR"));
        assert_eq!("GBP", universal_asset_code("ZGBP"));
        assert_eq!("JPY", universal_asset_code("ZJPY"));
        assert_eq!("USD", universal_asset_code("ZUSD"));

        assert_eq!("USDC", universal_asset_code("USDC"));
        assert_eq!("USDT", universal_asset_code("USDT"));
    }
}
