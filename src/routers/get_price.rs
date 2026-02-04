use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde::Serialize;
use sqlx::{PgPool, Row, query};

#[derive(Serialize)]
pub struct ProductPrice {
    pub id: i32,
    pub current_price: f64,
}

pub async fn get_product_price(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<ProductPrice>, StatusCode> {
    let product = query("SELECT id, current_price FROM products WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let current_price_decimal: String = product
        .try_get("current_price")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let current_price: f64 = current_price_decimal
        .parse()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ProductPrice { id, current_price }))
}
