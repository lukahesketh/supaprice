// everytime someone checks out we send a request here
//
//
use crate::data_structures::order::OrderStruct;
use axum::extract::Json;
use axum::extract::State;
use sqlx::PgPool;
use sqlx::query;

pub async fn init_order(State(pool): State<PgPool>, Json(order): Json<OrderStruct>) {
    query("INSERT INTO purchases (product_id, quantity) VALUES ($1, $2)")
        .bind(&order.product_id)
        .bind(&order.quantity)
        .execute(&pool)
        .await
        .unwrap();
}
