mod data_structures;
mod routers;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let connected_to_pool = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&connected_to_pool).await.unwrap();

    let pool_for_worker = pool.clone();

    let app = routers::router(pool);

    tokio::spawn(async move {
        routers::calculate_price::calculate_price(pool_for_worker).await;
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3069").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
