use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub sku: String,
    pub product_name: String,
    pub category: String,
    pub ean13: String,
    pub price_cents: i32,
    pub currency: String,
    pub stock_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/*
id (uuid) PK
sku text unique not null
product_name text not null
category text not null
ean13 varchar(13) unique not null
price_cents int not null
currency char(3) default 'BRL'
stock_count int not null
created_at timestamptz/datetime default now
updated_at timestamptz/datetime default now
*/