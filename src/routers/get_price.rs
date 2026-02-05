use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
pub struct ProductPrice {
    pub id: i32,
    pub current_price: f64,
}

pub async fn get_product_price(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<ProductPrice>, StatusCode> {
    let product =
        sqlx::query_as::<_, ProductPrice>("SELECT id, current_price FROM products WHERE id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(product))
}
