use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method};
use axum::{Router, routing::post};
use tower_http::cors::CorsLayer;
mod init_store;
use sqlx::PgPool;

use crate::routers::init_order::init_order;
mod init_order;
mod init_product;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/init_store", post(init_store::init_store))
        .route("/init_product", post(init_product::init_product))
        .route("/init_order", post(init_order::init_order))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([CONTENT_TYPE]),
        )
        .with_state(pool)
}
