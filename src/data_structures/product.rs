use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Product {
    pub store_id: i32,
    pub external_id: String,
    pub baseline_price: f64,
    pub current_price: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub sensitivity: f64,
    pub stock: i32,
    pub update_interval_minutes: i32,
}

// we need a database table for a speicifc shop, storing there domain and there specifc products
// and those products need a min price and max price, and a volatility setting.
