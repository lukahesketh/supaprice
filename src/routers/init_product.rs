use crate::data_structures::product::Product;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use sqlx::PgPool;
use sqlx::query;

pub async fn init_product(
    State(pool): State<PgPool>,
    Json(product_init): Json<Product>,
) -> StatusCode {
    query("INSERT INTO products (store_id, external_id, baseline_price, min_price, max_price, sensitivity, stock, update_interval_minutes) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(&product_init.store_id)
        .bind(&product_init.external_id)
        .bind(&product_init.baseline_price)
        .bind(&product_init.min_price)
        .bind(&product_init.max_price)
        .bind(&product_init.sensitivity)
        .bind(&product_init.stock)
        .bind(&product_init.update_interval_minutes)
        .execute(&pool)
        .await
        .unwrap();

    StatusCode::CREATED
}
