use crate::api::prelude::*;
use crate::api::trading_pair::TradingPairInfo;
use crate::api::RL_GENERAL_KEY;

pub type ListTradingPairInfoResponse = Vec<TradingPairInfo>;

#[cfg(feature = "with_network")]
impl<S> Api<S>
where
    S: crate::client::BitstampSigner,
    S: Unpin + 'static,
{
    /// Trading pairs info
    ///
    /// [https://www.bitstamp.net/api/#trading-pairs-info]
    pub fn list_trading_pairs(&self) -> BitstampResult<Task<ListTradingPairInfoResponse>> {
        let endpoint = "trading-pairs-info/";

        Ok(self
            .rate_limiter
            .task(self.client.get(endpoint)?.request_body(())?)
            .cost(RL_GENERAL_KEY, 1)
            .send())
    }
}
