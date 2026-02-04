use crate::data_structures::store::{CreateStoreStruct, Store};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use sqlx::PgPool;

pub async fn init_store(
    State(pool): State<PgPool>,
    Json(store_info): Json<CreateStoreStruct>,
) -> Result<Json<Store>, StatusCode> {
    let store = sqlx::query_as::<_, Store>(
        "INSERT INTO stores (domain, business_name) VALUES ($1, $2) RETURNING id, domain, business_name",
    )
    .bind(&store_info.domain)
    .bind(&store_info.business_name)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(store))
}
