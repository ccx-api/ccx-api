use super::super::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct OrderFilters {
    pub price: Option<PriceFilter>,
    pub percent_price: Option<PercentPriceFilter>,
    pub lot_size: Option<LotSizeFilter>,
    pub min_notional: Option<MinNotionalFilter>,
    pub icebert_parts: Option<IcebergPartsFilter>,
    pub market_lot_size: Option<MarketLotSizeFilter>,
    pub max_num_orders: Option<MaxNumOrdersFilter>,
    pub max_num_algo_orders: Option<MaxNumAlgoOrdersFilter>,
    pub max_num_iceberg: Option<MaxNumIcebergOrdersFilter>,
    pub max_position: Option<MaxPositionFilter>,
}

impl OrderFilters {
    pub fn from_filters(filters: &[Filter]) -> Self {
        let mut this = OrderFilters::default();
        for &filter in filters {
            match filter {
                Filter::Price(filter) => this.price = Some(filter),
                Filter::PercentPrice(filter) => this.percent_price = Some(filter),
                Filter::LotSize(filter) => this.lot_size = Some(filter),
                Filter::MinNotional(filter) => this.min_notional = Some(filter),
                Filter::IcebergParts(filter) => this.icebert_parts = Some(filter),
                Filter::MarketLotSize(filter) => this.market_lot_size = Some(filter),
                Filter::MaxNumOrders(filter) => this.max_num_orders = Some(filter),
                Filter::MaxNumAlgoOrders(filter) => this.max_num_algo_orders = Some(filter),
                Filter::MaxNumIcebergOrders(filter) => this.max_num_iceberg = Some(filter),
                Filter::MaxPosition(filter) => this.max_position = Some(filter),
            }
        }
        this
    }
}
