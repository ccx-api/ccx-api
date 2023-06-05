use crate::api::prelude::*;
use crate::api::trading_pair::MyTradingPair;
use crate::api::RL_GENERAL_KEY;

pub type MyTradingPairResponse = Vec<MyTradingPair>;

#[cfg(feature = "with_network")]
impl<S> Api<S>
where
    S: crate::client::BitstampSigner,
    S: Unpin + 'static,
{
    /// My trading pairs
    ///
    /// Returns all trading pairs that can be traded on selected account.
    ///
    /// [https://www.bitstamp.net/api/#trading-pairs]
    pub fn my_trading_pairs(&self) -> BitstampResult<Task<MyTradingPairResponse>> {
        let endpoint = "my_trading_pairs/";

        Ok(self
            .rate_limiter
            .task(self.client.post(endpoint)?.signed_now()?.request_body(())?)
            .cost(RL_GENERAL_KEY, 1)
            .send())
    }
}
