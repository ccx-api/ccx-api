use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

use crate::types::enums::GoodsCategory;
use crate::types::enums::GoodsType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderGoods {
    #[serde(rename = "goodsType")]
    pub type_: GoodsType,
    #[serde(rename = "goodsCategory")]
    pub category: GoodsCategory,
    #[serde(rename = "referenceGoodsId")]
    pub reference_id: String,
    #[serde(rename = "goodsName")]
    pub name: String,
    #[serde(rename = "goodsDetail")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "goodsUnitAmount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<GoodsUnitAmount>,
    #[serde(rename = "goodsQuantity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
}

impl Default for OrderGoods {
    fn default() -> Self {
        let id = uuid::Uuid::new_v4();
        Self {
            type_: GoodsType::default(),
            category: GoodsCategory::default(),
            reference_id: id.to_string(),
            name: format!("Goods name for {}", id),
            detail: None::<String>,
            unit_amount: None::<GoodsUnitAmount>,
            quantity: None::<String>,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoodsUnitAmount {
    pub currency: String,
    pub amount: Decimal,
}
