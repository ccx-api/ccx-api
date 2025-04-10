use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use serde::{Deserialize, Deserializer};

pub struct Fill {
    pub base_value: Decimal,
    pub quote_value: Decimal,
    pub exhausted: bool,
}

impl Fill {
    pub fn price(&self) -> Decimal {
        self.quote_value / self.base_value
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct PriceAndAmount {
    pub price: Decimal,
    pub amount: Decimal,
}

impl<'de> Deserialize<'de> for PriceAndAmount {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let [price, amount] = <[Decimal; 2]>::deserialize(deserializer)?;
        Ok(Self { price, amount })
    }
}

impl From<(Decimal, Decimal)> for PriceAndAmount {
    fn from((price, amount): (Decimal, Decimal)) -> Self {
        Self { price, amount }
    }
}

impl From<(&Decimal, &Decimal)> for PriceAndAmount {
    fn from((&price, &amount): (&Decimal, &Decimal)) -> Self {
        Self { price, amount }
    }
}

pub trait OrderBook {
    /// WARN: when implementing, the asks should be ordered from low to high ask
    fn asks(&self) -> impl ExactSizeIterator<Item = PriceAndAmount>;
    /// WARN: when implementing, the bids should be ordered from high to low bid
    fn bids(&self) -> impl ExactSizeIterator<Item = PriceAndAmount>;

    fn bid_volume(&self, price_limit: Decimal) -> Fill {
        let mut base_value = Decimal::zero();
        let mut quote_value = Decimal::zero();
        let mut exhausted = true;
        for PriceAndAmount { price, amount } in self.bids() {
            if price_limit < price {
                exhausted = false;
                break;
            }
            base_value += amount;
            quote_value += amount * price;
        }
        Fill {
            base_value,
            quote_value,
            exhausted,
        }
    }

    fn ask_volume(&self, price_limit: Decimal) -> Fill {
        let mut base_value = Decimal::zero();
        let mut quote_value = Decimal::zero();
        let mut exhausted = true;
        for PriceAndAmount { price, amount } in self.asks() {
            if price_limit > price {
                exhausted = false;
                break;
            }
            base_value += amount;
            quote_value += amount * price;
        }
        Fill {
            base_value,
            quote_value,
            exhausted,
        }
    }

    fn bid_base_depth(&self, base_qty_limit: Decimal) -> Fill {
        let mut base_value = Decimal::zero();
        let mut quote_value = Decimal::zero();
        let mut exhausted = true;
        for PriceAndAmount { price, amount } in self.bids() {
            let diff = base_qty_limit - base_value;
            let amount = diff.min(amount);
            base_value += amount;
            quote_value += amount * price;
            if amount == diff {
                exhausted = false;
                break;
            }
        }
        Fill {
            base_value,
            quote_value,
            exhausted,
        }
    }

    fn ask_base_depth(&self, base_qty_limit: Decimal) -> Fill {
        let mut base_value = Decimal::zero();
        let mut quote_value = Decimal::zero();
        let mut exhausted = true;
        for PriceAndAmount { price, amount } in self.asks() {
            let diff = base_qty_limit - base_value;
            let amount = diff.min(amount);
            base_value += amount;
            quote_value += amount * price;
            if amount == diff {
                exhausted = false;
                break;
            }
        }
        Fill {
            base_value,
            quote_value,
            exhausted,
        }
    }

    fn ask_quote_depth(&self, quote_qty_limit: Decimal) -> Fill {
        let mut base_value = Decimal::zero();
        let mut quote_value = Decimal::zero();
        let mut exhausted = true;

        for PriceAndAmount { price, amount } in self.asks() {
            let diff = quote_qty_limit - quote_value;
            let amount = (diff / price).min(amount);
            base_value += amount;
            quote_value += amount * price;

            if quote_value >= quote_qty_limit {
                exhausted = false;
                break;
            }
        }

        Fill {
            base_value,
            quote_value,
            exhausted,
        }
    }

    fn bid_quote_depth(&self, quote_qty_limit: Decimal) -> Fill {
        let mut base_value = Decimal::zero();
        let mut quote_value = Decimal::zero();
        let mut exhausted = true;

        for PriceAndAmount { price, amount } in self.bids() {
            let diff = quote_qty_limit - quote_value;
            let amount = (diff / price).min(amount);
            base_value += amount;
            quote_value += amount * price;

            if quote_value >= quote_qty_limit {
                exhausted = false;
                break;
            }
        }

        Fill {
            base_value,
            quote_value,
            exhausted,
        }
    }

    fn spread(&self) -> Option<Decimal> {
        let ask = self.asks().into_iter().next()?;
        let bid = self.bids().into_iter().next()?;
        Some(ask.price - bid.price)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use rust_decimal_macros::dec;

    struct TestOrderBook {
        bids: BTreeMap<Decimal, Decimal>,
        asks: BTreeMap<Decimal, Decimal>,
    }

    impl OrderBook for TestOrderBook {
        fn asks(&self) -> impl ExactSizeIterator<Item = PriceAndAmount> {
            self.asks.iter().map(PriceAndAmount::from)
        }

        fn bids(&self) -> impl ExactSizeIterator<Item = PriceAndAmount> {
            self.bids.iter().rev().map(PriceAndAmount::from)
        }
    }

    #[test]
    fn test_bid_base_depth() {
        let mut bids = BTreeMap::new();
        bids.insert(dec!(100.0), dec!(1.0));
        bids.insert(dec!(99.0), dec!(2.0));
        bids.insert(dec!(98.0), dec!(3.0));

        let order_book = TestOrderBook {
            bids,
            asks: BTreeMap::new(),
        };

        let fill = order_book.bid_base_depth(dec!(4.0));
        assert_eq!(fill.base_value, dec!(4.0));
        assert_eq!(fill.quote_value, dec!(396.0));
        assert!(!fill.exhausted);
        assert_eq!(fill.price(), dec!(99));
    }

    #[test]
    fn test_ask_base_depth() {
        let mut asks = BTreeMap::new();
        asks.insert(dec!(101.0), dec!(1.0));
        asks.insert(dec!(102.0), dec!(2.0));
        asks.insert(dec!(103.0), dec!(3.0));

        let order_book = TestOrderBook {
            bids: BTreeMap::new(),
            asks,
        };

        let fill = order_book.ask_base_depth(dec!(4.0));
        assert_eq!(fill.base_value, dec!(4.0));
        assert_eq!(fill.quote_value, dec!(408.0));
        assert!(!fill.exhausted);
        assert_eq!(fill.price(), dec!(102));
    }

    #[test]
    fn test_bid_quote_depth() {
        let mut bids = BTreeMap::new();
        bids.insert(dec!(99.0), dec!(3.0));
        bids.insert(dec!(100.0), dec!(2.0));
        bids.insert(dec!(101.0), dec!(1.0));

        let order_book = TestOrderBook {
            bids,
            asks: BTreeMap::new(),
        };

        let fill = order_book.bid_quote_depth(dec!(201.0));
        assert_eq!(fill.base_value, dec!(2));
        assert_eq!(fill.quote_value, dec!(201.0));
        assert!(!fill.exhausted);
        assert_eq!(fill.price(), dec!(100.5));
    }

    #[test]
    fn test_ask_quote_depth() {
        let mut asks = BTreeMap::new();
        asks.insert(dec!(99.0), dec!(1.0));
        asks.insert(dec!(100.0), dec!(2.0));
        asks.insert(dec!(101.0), dec!(3.0));

        let order_book = TestOrderBook {
            bids: BTreeMap::new(),
            asks,
        };

        let fill = order_book.ask_quote_depth(dec!(199.0));
        assert_eq!(fill.base_value, dec!(2));
        assert_eq!(fill.quote_value, dec!(199.0));
        assert!(!fill.exhausted);
        assert_eq!(fill.price(), dec!(99.5));
    }
}
