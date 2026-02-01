// everytime someone checks out we send a request here
//
//

use axum::extract::State;
use sqlx::PgPool;

pub async fn init_order(State(pool): State<PgPool>) {}
