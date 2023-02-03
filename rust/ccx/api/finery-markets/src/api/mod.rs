mod spot;

pub use spot::SpotApi;

#[cfg(test)]
mod tests {

    #![allow(unused_variables)]

    use std::time::Duration;

    use actix::clock::sleep;

    use crate::api::spot::SpotApi;
    use crate::error::LibResult;
    use crate::types::Time;
    use crate::ApiCred;

    use crate::types::CLimitsRequest;
    use crate::types::DelLimitRequest;
    use crate::types::InstrumentsRequest;
    use crate::types::LimitsRequest;
    use crate::types::PositionsRequest;
    use crate::types::SettlementRequest;
    use crate::types::SettlementTransactionsRequest;

    use crate::types::ClientId;
    use crate::types::ClientOrderId;
    use crate::types::DealId;
    use crate::types::NonceSeq;
    use crate::types::OrderId;
    use crate::types::OrderTypeByName;
    use crate::types::Pair;
    use crate::types::Price;
    use crate::types::SettlementFlags;
    use crate::types::SideByName;
    use crate::types::Size;
    use crate::types::Timestamp;

    use crate::types::AddIncomingSettlementRequest;
    use crate::types::AddOutgoingSettlementTransactionRequest;
    use crate::types::AddRequest;
    use crate::types::BookRequest;
    use crate::types::CommitIncomingSettlementTransactionRequest;
    use crate::types::DealHistoryRequest;
    use crate::types::DelAllRequest;
    use crate::types::DelCLimitRequest;
    use crate::types::DelIncomingSettlementCPRequest;
    use crate::types::DelIncomingSettlementRequest;
    use crate::types::DelOutgoingSettlementTransactionRequest;
    use crate::types::DelRequest;
    use crate::types::ModRequest;
    use crate::types::SendOutgoingSettlementTransactionRequest;
    use crate::types::SetCLimitRequest;
    use crate::types::SetLimitRequest;
    use crate::types::SettlementHistoryRequest;
    use crate::types::SettlementTransactionHistoryRequest;

    type Api = SpotApi<ApiCred>;

    const CLIENT_ID: ClientId = 321;
    const ORDER_ID: OrderId = 321;
    const TRANSACTION_ID: OrderId = 321;
    // const CLIENT_ORDER_ID: ClientOrderId = 321;
    const PRICE: Price = 321;
    const SIZE: Size = 1;
    const SIDE: SideByName = SideByName::Bid;
    const COD: bool = false;
    const METHOD_PAUSE: Duration = Duration::from_secs(1);

    async fn test_without_params_instruments(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = InstrumentsRequest {};
        log::debug!(
            "test_without_params_instruments :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api.instruments(nonce, time, request).await?;
        log::debug!("test_without_params_instruments :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_positions(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = PositionsRequest {};
        log::debug!("test_without_params_positions :: {:?} :: {:?}", time, nonce);
        let response = api.positions(nonce, time, request).await?;
        log::debug!("test_without_params_positions :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_limits(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = LimitsRequest {};
        log::debug!("test_without_params_limits :: {:?} :: {:?}", time, nonce);
        let response = api.limits(nonce, time, request).await?;
        log::debug!("test_without_params_limits :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_climits(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = CLimitsRequest {};
        log::debug!("test_without_params_climits :: {:?} :: {:?}", time, nonce);
        let response = api.climits(nonce, time, request).await?;
        log::debug!("test_without_params_climits :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_settlement_requests(
        api: &Api,
        seq: &mut NonceSeq,
    ) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = SettlementRequest {};
        log::debug!(
            "test_without_params_settlement_requests :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api.settlement_requests(nonce, time, request).await?;
        log::debug!("test_without_params_settlement_requests :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_settlement_transactions(
        api: &Api,
        seq: &mut NonceSeq,
    ) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = SettlementTransactionsRequest {};
        log::debug!(
            "test_without_params_settlement_transactions :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api.settlement_transactions(nonce, time, request).await?;
        log::debug!(
            "test_without_params_settlement_transactions :: {:#?}",
            response
        );
        Ok(())
    }

    async fn test_without_params_book(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = BookRequest {
            // instrument: Pair::new("BTC", "USD"),
            instrument: Pair::new("ETH", "BTC"),
            tradable: true,
        };
        log::debug!("test_without_params_book :: {:?} :: {:?}", time, nonce);
        let response = api.book(nonce, time, request).await?;
        log::debug!("test_without_params_book :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_deal_history(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = DealHistoryRequest {
            instrument: Pair::new("BTC", "USD"),
            till: Option::<DealId>::None,
            from: Option::<Timestamp>::None,
            to: Option::<Timestamp>::None,
            limit: Option::<u16>::None,
        };
        log::debug!(
            "test_without_params_deal_history :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api.deal_history(nonce, time, request).await?;
        log::debug!("test_without_params_deal_history :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_settlement_history(
        api: &Api,
        seq: &mut NonceSeq,
    ) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = SettlementHistoryRequest {
            // till: Option::<DealId>::Some(12345),
            till: Option::<DealId>::None,
            from: Option::<Timestamp>::None,
            to: Option::<Timestamp>::None,
            limit: Option::<u16>::Some(10),
            // limit: Option::<u16>::None,
        };
        log::debug!(
            "test_without_params_settlement_history :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api.settlement_history(nonce, time, request).await?;
        log::debug!("test_without_params_settlement_history :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_settlement_transaction_history(
        api: &Api,
        seq: &mut NonceSeq,
    ) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = SettlementTransactionHistoryRequest {
            // till: Option::<DealId>::Some(12345),
            till: Option::<DealId>::None,
            from: Option::<Timestamp>::None,
            to: Option::<Timestamp>::None,
            // limit: Option::<u16>::Some(10),
            limit: Option::<u16>::None,
        };
        log::debug!(
            "test_without_params_settlement_transaction_history :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api
            .settlement_transaction_history(nonce, time, request)
            .await?;
        log::debug!(
            "test_without_params_settlement_transaction_history :: {:#?}",
            response
        );
        Ok(())
    }

    async fn test_without_params_add_taker(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;

        let pair = Pair::new("BTC", "USD");

        let book = {
            let time = Time::now();
            let nonce = seq.next();
            let request = BookRequest {
                instrument: pair.clone(),
                tradable: true,
            };
            log::debug!("test_without_params_add :: {:?} :: {:?}", time, nonce);
            api.book(nonce, time, request).await?
        };

        let price_ask = book.current_market_ask();
        log::debug!("test_without_params_add price_ask :: {:?}", price_ask);
        let price_bid = book.current_market_bid();
        log::debug!("test_without_params_add price_bid :: {:?}", price_bid);

        let time = Time::now();
        let nonce = seq.next();
        let request = AddRequest {
            instrument: pair,
            client_order_id: Some(Time::now().0),
            // price: PRICE,
            // price: Option::<Price>::Some(price_ask.expect("Failed price_bid").price),
            price: Option::<Price>::None,
            size: Option::<Size>::None,
            // size: Option::<Size>::Some(10_000_000),
            // volume: Option::<Size>::None,
            volume: Option::<Size>::Some(1_000_000),
            side: SideByName::Bid,
            r#type: OrderTypeByName::MarketFOK,
            cod: COD,
        };
        log::debug!("test_without_params_add :: {:?} :: {:?}", time, nonce);
        let response = api.add(nonce, time, request).await?;
        log::debug!("test_without_params_add :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_add_maker(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;

        let pair = Pair::new("BTC", "USD");

        let book = {
            let time = Time::now();
            let nonce = seq.next();
            let request = BookRequest {
                instrument: pair.clone(),
                tradable: true,
            };
            log::debug!("test_without_params_add :: {:?} :: {:?}", time, nonce);
            api.book(nonce, time, request).await?
        };

        let price_ask = book.last_market_ask();
        log::debug!("test_without_params_add price_ask :: {:?}", price_ask);
        let price_bid = book.current_market_bid();
        log::debug!("test_without_params_add price_bid :: {:?}", price_bid);

        let time = Time::now();
        let nonce = seq.next();
        let request = AddRequest {
            instrument: pair,
            client_order_id: Some(Time::now().0),
            // price: PRICE,
            // price: Option::<Price>::Some(price_ask.expect("Failed price_bid").price),
            price: Option::<Price>::None,
            // size: Option::<Size>::None,
            size: Option::<Size>::Some(10),
            volume: Option::<Size>::None,
            // volume: Option::<Size>::Some(1_000_000),
            side: SideByName::Bid,
            r#type: OrderTypeByName::PostOnly,
            cod: COD,
        };
        log::debug!("test_without_params_add :: {:?} :: {:?}", time, nonce);
        let response = api.add(nonce, time, request).await?;
        log::debug!("test_without_params_add :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_call_mod(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let book = {
            let time = Time::now();
            let nonce = seq.next();
            let request = BookRequest {
                instrument: Pair::new("BTC", "USD"),
                tradable: true,
            };
            log::debug!("test_without_params_add :: {:?} :: {:?}", time, nonce);
            api.book(nonce, time, request).await?
        };

        let price_ask = book.current_market_ask();
        log::debug!("test_without_params_call_mod price_ask :: {:?}", price_ask);
        let price_bid = book.current_market_bid();
        log::debug!("test_without_params_call_mod price_bid :: {:?}", price_bid);

        let time = Time::now();
        let nonce = seq.next();
        let request = ModRequest {
            order_id: 14348564747,
            client_order_id: Option::<ClientOrderId>::Some(1643630746283),
            price: price_bid.expect("Failed price_bid").price,
            size: 3731700000000 + 2_000_000,
        };
        log::debug!("test_without_params_call_mod :: {:?} :: {:?}", time, nonce);
        let response = api.call_mod(nonce, time, request).await?;
        log::debug!("test_without_params_call_mod :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_del(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = DelRequest {
            // order_id: Option::<OrderId>::None,
            order_id: Option::<OrderId>::Some(1643630746283),
            client_order_id: Option::<ClientOrderId>::None,
            // client_order_id: Option::<ClientOrderId>::Some(1643630746283),
        };
        log::debug!(
            "test_without_params_del :: {:?} :: {:?} :: {:?}",
            time,
            nonce,
            request
        );
        let response = api.del(nonce, time, request).await?;
        log::debug!("test_without_params_del :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_del_all(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = DelAllRequest {
            instrument: Option::<Pair>::Some(Pair::new("BTC", "USD")),
        };
        log::debug!("test_without_params_del_all :: {:?} :: {:?}", time, nonce);
        let response = api.del_all(nonce, time, request).await?;
        log::debug!("test_without_params_del_all :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_add_incoming_settlement_request(
        api: &Api,
        seq: &mut NonceSeq,
    ) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = AddIncomingSettlementRequest {
            counterparty_id: 20,
            currency: String::from("GBP"),
            amount: Option::<Size>::Some(10_000_000),
            comment: Option::<String>::Some(format!("Some text :: {}", Time::now().0)),
            flags: Option::<SettlementFlags>::Some(SettlementFlags::NoFlags),
            cancel_timestamp: Option::<Timestamp>::Some(Time::now().0 + 3600 * 1000),
        };
        log::debug!(
            "test_without_params_add_incoming_settlement_request :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api
            .add_incoming_settlement_request(nonce, time, request)
            .await?;
        log::debug!(
            "test_without_params_add_incoming_settlement_request :: {:#?}",
            response
        );
        Ok(())
    }

    async fn test_without_params_del_incoming_settlement_request(
        api: &Api,
        seq: &mut NonceSeq,
    ) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = DelIncomingSettlementRequest {
            counterparty_id: 87,
            currency: String::from("USD"),
        };
        log::debug!(
            "test_without_params_del_incoming_settlement_request :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api
            .del_incoming_settlement_request(nonce, time, request)
            .await?;
        log::debug!(
            "test_without_params_del_incoming_settlement_request :: {:#?}",
            response
        );
        Ok(())
    }

    async fn test_without_params_del_incoming_settlement_cp_request(
        api: &Api,
        seq: &mut NonceSeq,
    ) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = DelIncomingSettlementCPRequest {
            counterparty_id: 87,
            currency: String::from("BTC"),
        };
        log::debug!(
            "test_without_params_del_incoming_settlement_cp_request :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api
            .del_incoming_settlement_cp_request(nonce, time, request)
            .await?;
        log::debug!(
            "test_without_params_del_incoming_settlement_cp_request :: {:#?}",
            response
        );
        Ok(())
    }

    async fn test_without_params_add_outgoing_settlement_transaction(
        api: &Api,
        seq: &mut NonceSeq,
    ) -> LibResult<DealId> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let currency = "BNB";
        // let amount = 2600_000_000;
        let amount = 27;
        let request = AddOutgoingSettlementTransactionRequest {
            counterparty_id: 306,
            currency: String::from(currency),
            amount,
            comment: format!("I am going to send {} {} transaction", amount, currency),
            fee: Option::<Size>::Some(1),
        };
        log::debug!(
            "test_without_params_add_outgoing_settlement_transaction :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api
            .add_outgoing_settlement_transaction(nonce, time, request)
            .await?;
        log::debug!(
            "test_without_params_add_outgoing_settlement_transaction :: {:#?}",
            response
        );
        Ok(response.settlement_transaction_id)
    }

    async fn test_without_params_send_outgoing_settlement_transaction(
        api: &Api,
        seq: &mut NonceSeq,
        transaction_id: DealId,
    ) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        // let transaction_id = 15153840079;
        let request = SendOutgoingSettlementTransactionRequest {
            transaction_id,
            tx_id: format!("This is tx id: {}", transaction_id),
        };
        log::debug!(
            "test_without_params_send_outgoing_settlement_transaction :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api
            .send_outgoing_settlement_transaction(nonce, time, request)
            .await?;
        log::debug!(
            "test_without_params_send_outgoing_settlement_transaction :: {:#?}",
            response
        );
        Ok(())
    }

    async fn test_without_params_commit_incoming_settlement_transaction(
        api: &Api,
        seq: &mut NonceSeq,
    ) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = CommitIncomingSettlementTransactionRequest {
            transaction_id: 14348679512,
        };
        log::debug!(
            "test_without_params_commit_incoming_settlement_transaction :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api
            .commit_incoming_settlement_transaction(nonce, time, request)
            .await?;
        log::debug!(
            "test_without_params_commit_incoming_settlement_transaction :: {:#?}",
            response
        );
        Ok(())
    }

    async fn test_without_params_del_outgoing_settlement_transaction(
        api: &Api,
        seq: &mut NonceSeq,
    ) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = DelOutgoingSettlementTransactionRequest {
            transaction_id: 14347366171,
        };
        log::debug!(
            "test_without_params_del_outgoing_settlement_transaction :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api
            .del_outgoing_settlement_transaction(nonce, time, request)
            .await?;
        log::debug!(
            "test_without_params_del_outgoing_settlement_transaction :: {:#?}",
            response
        );
        Ok(())
    }

    async fn test_without_params_set_limit(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = SetLimitRequest {
            currency: String::from("EUR"),
            net_limit: 10000000000000,
            gross_limit: 15000000000000,
        };
        log::debug!("test_without_params_set_limit :: {:?} :: {:?}", time, nonce);
        let response = api.set_limit(nonce, time, request).await?;
        log::debug!("test_without_params_set_limit :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_del_limit(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = DelLimitRequest {};
        log::debug!("test_without_params_del_limit :: {:?} :: {:?}", time, nonce);
        let response = api.del_limit(nonce, time, request).await?;
        log::debug!("test_without_params_del_limit :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_set_climit(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = SetCLimitRequest {
            counterparty_id: 4,
            currency: String::from("EUR"),
            net_limit: 10000000000000,
            gross_limit: 15000000000000,
            taker_markup: Option::<i32>::Some(100),
        };
        log::debug!(
            "test_without_params_set_climit :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api.set_climit(nonce, time, request).await?;
        log::debug!("test_without_params_set_climit :: {:#?}", response);
        Ok(())
    }

    async fn test_without_params_del_climit(api: &Api, seq: &mut NonceSeq) -> LibResult<()> {
        sleep(METHOD_PAUSE).await;
        let time = Time::now();
        let nonce = seq.next();
        let request = DelCLimitRequest {
            counterparty_id: CLIENT_ID,
        };
        log::debug!(
            "test_without_params_del_climit :: {:?} :: {:?}",
            time,
            nonce
        );
        let response = api.del_climit(nonce, time, request).await?;
        log::debug!("test_without_params_del_climit :: {:#?}", response);
        Ok(())
    }
}
