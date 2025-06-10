use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use rust_decimal_macros::dec;
use serde::{Deserialize, Deserializer};

// Use 10 significant digits after floating point
const MIN_THRESHOLD: Decimal = dec!(0.0000000001);

#[derive(Debug, Clone, PartialEq, derive_more::Display, derive_more::Error)]
#[display("Liquidity exhausted")]
pub struct ExhaustedError {
    pub filled: Fill,
}

type FillResult = Result<Fill, ExhaustedError>;

#[derive(Debug, Clone, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fill {
    pub base_value: Decimal,
    pub quote_value: Decimal,
    pub max_base_price: Decimal,
    pub min_base_price: Decimal,
    pub side: OrderSide,
}

impl Fill {
    pub fn price(&self) -> Decimal {
        self.quote_value / self.base_value
    }

    pub fn slippage(&self) -> Decimal {
        let divisor = match self.side {
            OrderSide::Buy => self.min_base_price,
            OrderSide::Sell => self.max_base_price,
        };
        (self.max_base_price - self.min_base_price) / divisor
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

fn base_depth(
    side: OrderSide,
    entries: impl ExactSizeIterator<Item = PriceAndAmount>,
    base_qty_limit: Decimal,
) -> FillResult {
    let mut base_value = Decimal::zero();
    let mut quote_value = Decimal::zero();
    let mut max_base_price = Decimal::ZERO;
    let mut min_base_price = Decimal::MAX;

    for PriceAndAmount { price, amount } in entries {
        let diff = base_qty_limit - base_value;
        let amount = diff.min(amount);
        base_value += amount;
        max_base_price = max_base_price.max(price);
        min_base_price = min_base_price.min(price);
        quote_value += amount * price;
        if amount == diff {
            return Ok(Fill {
                side,
                base_value,
                quote_value,
                max_base_price,
                min_base_price,
            });
        }
    }

    Err(ExhaustedError {
        filled: Fill {
            side,
            base_value,
            quote_value,
            max_base_price,
            min_base_price,
        },
    })
}

fn quote_depth(
    side: OrderSide,
    entries: impl ExactSizeIterator<Item = PriceAndAmount>,
    quote_qty_limit: Decimal,
) -> FillResult {
    let mut base_value = Decimal::zero();
    let mut quote_value = Decimal::zero();
    let mut max_base_price = Decimal::ZERO;
    let mut min_base_price = Decimal::MAX;

    for PriceAndAmount { price, amount } in entries {
        let diff = quote_qty_limit - quote_value;
        let amount = (diff / price).min(amount);
        base_value += amount;
        quote_value += amount * price;
        max_base_price = max_base_price.max(price);
        min_base_price = min_base_price.min(price);

        if (quote_value - quote_qty_limit).abs() < MIN_THRESHOLD {
            return Ok(Fill {
                side,
                base_value,
                quote_value,
                max_base_price,
                min_base_price,
            });
        }
    }

    Err(ExhaustedError {
        filled: Fill {
            side,
            base_value,
            quote_value,
            max_base_price,
            min_base_price,
        },
    })
}

pub trait OrderBook {
    /// WARN: when implementing, the asks should be ordered from low to high ask
    fn asks(&self) -> impl ExactSizeIterator<Item = PriceAndAmount>;
    /// WARN: when implementing, the bids should be ordered from high to low bid
    fn bids(&self) -> impl ExactSizeIterator<Item = PriceAndAmount>;

    fn bid_volume(&self, price_limit: Decimal) -> FillResult {
        let mut base_value = Decimal::zero();
        let mut quote_value = Decimal::zero();
        let mut max_base_price = Decimal::ZERO;
        let mut min_base_price = Decimal::MAX;

        for PriceAndAmount { price, amount } in self.bids() {
            if price < price_limit {
                return Ok(Fill {
                    side: OrderSide::Sell,
                    base_value,
                    quote_value,
                    max_base_price,
                    min_base_price,
                });
            }
            base_value += amount;
            quote_value += amount * price;
            max_base_price = max_base_price.max(price);
            min_base_price = min_base_price.min(price);
        }

        Err(ExhaustedError {
            filled: Fill {
                side: OrderSide::Sell,
                base_value,
                quote_value,
                max_base_price,
                min_base_price,
            },
        })
    }

    fn ask_volume(&self, price_limit: Decimal) -> FillResult {
        let mut base_value = Decimal::zero();
        let mut quote_value = Decimal::zero();
        let mut max_base_price = Decimal::ZERO;
        let mut min_base_price = Decimal::MAX;

        for PriceAndAmount { price, amount } in self.asks() {
            if price > price_limit {
                return Ok(Fill {
                    side: OrderSide::Buy,
                    base_value,
                    quote_value,
                    max_base_price,
                    min_base_price,
                });
            }
            base_value += amount;
            quote_value += amount * price;
            max_base_price = max_base_price.max(price);
            min_base_price = min_base_price.min(price);
        }

        Err(ExhaustedError {
            filled: Fill {
                side: OrderSide::Buy,
                base_value,
                quote_value,
                max_base_price,
                min_base_price,
            },
        })
    }

    fn ask_base_depth(&self, base_qty_limit: Decimal) -> FillResult {
        base_depth(OrderSide::Buy, self.asks(), base_qty_limit)
    }

    fn bid_base_depth(&self, base_qty_limit: Decimal) -> FillResult {
        base_depth(OrderSide::Sell, self.bids(), base_qty_limit)
    }

    fn ask_quote_depth(&self, quote_qty_limit: Decimal) -> FillResult {
        quote_depth(OrderSide::Buy, self.asks(), quote_qty_limit)
    }

    fn bid_quote_depth(&self, quote_qty_limit: Decimal) -> FillResult {
        quote_depth(OrderSide::Sell, self.bids(), quote_qty_limit)
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
    fn empty_order_book() {
        let order_book = TestOrderBook {
            bids: Default::default(),
            asks: Default::default(),
        };

        let error = order_book.ask_quote_depth(dec!(0.001)).unwrap_err();

        assert_eq!(
            error.filled,
            Fill {
                side: OrderSide::Buy,
                base_value: Decimal::ZERO,
                quote_value: Decimal::ZERO,
                min_base_price: Decimal::MAX,
                max_base_price: Decimal::ZERO,
            }
        );
    }

    #[test]
    fn test_bid_volume() {
        let mut bids = BTreeMap::new();
        bids.insert(dec!(100.0), dec!(1.0));
        bids.insert(dec!(99.0), dec!(2.0));
        bids.insert(dec!(98.0), dec!(3.0));

        let order_book = TestOrderBook {
            bids,
            asks: BTreeMap::new(),
        };

        let fill = order_book.bid_volume(dec!(99)).unwrap();

        assert_eq!(
            fill,
            Fill {
                side: OrderSide::Sell,
                base_value: dec!(3.0),
                quote_value: dec!(298.0),
                min_base_price: dec!(99),
                max_base_price: dec!(100),
            }
        );

        assert_eq!(fill.slippage(), dec!(0.01));
    }

    #[test]
    fn test_ask_volume() {
        let mut asks = BTreeMap::new();

        asks.insert(dec!(98.0), dec!(3.0));
        asks.insert(dec!(99.0), dec!(2.0));
        asks.insert(dec!(100.0), dec!(1.0));

        let order_book = TestOrderBook {
            bids: Default::default(),
            asks,
        };

        let fill = order_book.ask_volume(dec!(98.5)).unwrap();

        assert_eq!(
            fill,
            Fill {
                side: OrderSide::Buy,
                base_value: dec!(3.0),
                quote_value: dec!(294.0),
                min_base_price: dec!(98),
                max_base_price: dec!(98),
            }
        );

        assert_eq!(fill.slippage(), Decimal::ZERO);
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

        let fill = order_book.bid_base_depth(dec!(4.0)).unwrap();

        assert_eq!(
            fill,
            Fill {
                side: OrderSide::Sell,
                base_value: dec!(4.0),
                quote_value: dec!(396.0),
                min_base_price: dec!(98),
                max_base_price: dec!(100),
            }
        );

        assert_eq!(fill.price(), dec!(99));
        assert_eq!(fill.slippage(), dec!(0.02));
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

        let fill = order_book.ask_base_depth(dec!(4.0)).unwrap();

        assert_eq!(
            fill,
            Fill {
                side: OrderSide::Buy,
                base_value: dec!(4.0),
                quote_value: dec!(408.0),
                min_base_price: dec!(101),
                max_base_price: dec!(103),
            }
        );

        assert_eq!(fill.price(), dec!(102));
        assert_eq!(fill.slippage(), dec!(0.0198019801980198019801980198));
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

        let fill = order_book.bid_quote_depth(dec!(201.0)).unwrap();
        assert_eq!(
            fill,
            Fill {
                side: OrderSide::Sell,
                base_value: dec!(2),
                quote_value: dec!(201.0),
                min_base_price: dec!(100),
                max_base_price: dec!(101),
            }
        );
        assert_eq!(fill.price(), dec!(100.5));
        assert_eq!(fill.slippage(), dec!(0.0099009900990099009900990099));
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

        let fill = order_book.ask_quote_depth(dec!(199.0)).unwrap();
        assert_eq!(
            fill,
            Fill {
                side: OrderSide::Buy,
                base_value: dec!(2),
                quote_value: dec!(199.0),
                min_base_price: dec!(99),
                max_base_price: dec!(100),
            }
        );
        assert_eq!(fill.price(), dec!(99.5));
        assert_eq!(fill.slippage(), dec!(0.0101010101010101010101010101));
    }

    #[test]
    fn test_ask_quote_depth_precision() {
        let mut asks = BTreeMap::new();
        asks.insert(dec!(71905.36), dec!(0.02751));
        asks.insert(dec!(71907.34), dec!(0.00636));
        asks.insert(dec!(71908.65), dec!(0.01138));

        let order_book = TestOrderBook {
            bids: BTreeMap::new(),
            asks,
        };

        let fill = order_book.ask_quote_depth(dec!(30.0)).unwrap();
        assert_eq!(
            fill,
            Fill {
                side: OrderSide::Buy,
                base_value: dec!(0.0004172150727011171350786645),
                quote_value: dec!(29.999999999999999999999999192),
                min_base_price: dec!(71905.36),
                max_base_price: dec!(71905.36),
            }
        );
        assert_eq!(fill.price(), dec!(71905.360000000000000000000001));
        assert_eq!(fill.slippage(), Decimal::ZERO);
    }

    #[test]
    fn test_ask_base_depth_precision() {
        let mut asks = BTreeMap::new();
        asks.insert(dec!(71905.36), dec!(0.02751));
        asks.insert(dec!(71907.34), dec!(0.00636));
        asks.insert(dec!(71908.65), dec!(0.01138));

        let order_book = TestOrderBook {
            bids: BTreeMap::new(),
            asks,
        };

        let fill = order_book.ask_base_depth(dec!(0.000417)).unwrap();
        assert_eq!(
            fill,
            Fill {
                side: OrderSide::Buy,
                base_value: dec!(0.000417),
                quote_value: dec!(29.98453512),
                min_base_price: dec!(71905.36),
                max_base_price: dec!(71905.36),
            }
        );
        assert_eq!(fill.price(), dec!(71905.36));
        assert_eq!(fill.slippage(), Decimal::ZERO);
    }

    #[test]
    fn test_ask_base_exhausted() {
        let mut asks = BTreeMap::new();
        asks.insert(dec!(71905.36), dec!(0.02751));
        asks.insert(dec!(71907.34), dec!(0.00636));
        asks.insert(dec!(71908.65), dec!(0.01138));

        let order_book = TestOrderBook {
            bids: BTreeMap::new(),
            asks,
        };

        let error = order_book.ask_base_depth(dec!(1)).unwrap_err();
        assert_eq!(
            error.filled,
            Fill {
                side: OrderSide::Buy,
                base_value: dec!(0.04525),
                quote_value: dec!(3253.7675730),
                min_base_price: dec!(71905.36),
                max_base_price: dec!(71908.65),
            }
        );
    }

    #[test]
    fn test_ask_quote_exhausted() {
        let mut asks = BTreeMap::new();
        asks.insert(dec!(71905.36), dec!(0.02751));
        asks.insert(dec!(71907.34), dec!(0.00636));
        asks.insert(dec!(71908.65), dec!(0.01138));

        let order_book = TestOrderBook {
            bids: BTreeMap::new(),
            asks,
        };

        let error = order_book.ask_quote_depth(dec!(5000)).unwrap_err();
        assert_eq!(
            error.filled,
            Fill {
                side: OrderSide::Buy,
                base_value: dec!(0.04525),
                quote_value: dec!(3253.7675730),
                min_base_price: dec!(71905.36),
                max_base_price: dec!(71908.65),
            }
        );
    }
}
