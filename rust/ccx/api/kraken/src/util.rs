// {
// 'XBT': 'BTC',
// 'BCC': 'BCH',
// 'DRK': 'DASH',
// 'BCHABC': 'BCH',
// 'BCHSV': 'BSV',
// }

use std::borrow::Cow;

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
