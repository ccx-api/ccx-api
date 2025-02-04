// use rust_decimal::Decimal;
// use string_cache::DefaultAtom as Atom;

//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct Transaction {
//    pub symbol: String,
//    pub order_id: u64,
//    pub client_order_id: String,
//    pub transact_time: u64,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct UserDataStream {
//    pub listen_key: String,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//pub struct Success {}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//#[serde(untagged)]
//pub enum Prices {
//    AllPrices(Vec<SymbolPrice>),
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//pub struct SymbolPrice {
//    pub symbol: String,
//    #[serde(with = "string_or_float")] pub price: f64,
//}
//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//#[serde(untagged)]
//pub enum BookTickers {
//    AllBookTickers(Vec<Tickers>),
//}
//
//#[derive(Debug, Clone)]
//pub enum KlineSummaries {
//    AllKlineSummaries(Vec<KlineSummary>),
//}
//
