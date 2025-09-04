use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateProduct {
    pub sku: String,
    pub product_name: String,
    pub category: String,
    pub ean13: String,
    pub price_cents: i32,
    pub currency: Option<String>,
    pub stock_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProduct {
    pub sku: Option<String>,
    pub product_name: Option<String>,
    pub category: Option<String>,
    pub ean13: Option<String>,
    pub price_cents: Option<i32>,
    pub currency: Option<String>,
    pub stock_count: Option<i32>,
}
