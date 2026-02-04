use sqlx::PgPool;
use sqlx::Row;
use sqlx::query;
use tokio::time::{Duration, interval};

pub async fn calculate_price(pool: PgPool) {
    let mut interval = interval(Duration::from_secs(150));

    loop {
        interval.tick().await;
        let products = query("SELECT id, stock, update_interval_minutes, current_price, baseline_price, min_price, max_price, sensitivity FROM products WHERE last_price_update IS NULL OR last_price_update + (update_interval_minutes || ' minutes')::interval <= NOW()")
            .fetch_all(&pool)
            .await
            .unwrap();

        for product in products {
            let id: i32 = product.try_get("id").unwrap();
            let stock: i32 = product.try_get("stock").unwrap();
            let interval_minutes: i32 = product.try_get("update_interval_minutes").unwrap();
            let current_price: f64 = product.try_get("current_price").unwrap();
            let baseline_price: f64 = product.try_get("baseline_price").unwrap();
            let min_price: f64 = product.try_get("min_price").unwrap();
            let max_price: f64 = product.try_get("max_price").unwrap();
            let sensitivity: f64 = product.try_get("sensitivity").unwrap();

            edit_price(
                &pool,
                id,
                stock,
                interval_minutes,
                current_price,
                baseline_price,
                min_price,
                max_price,
                sensitivity,
            )
            .await;
        }
    }
}

pub async fn edit_price(
    pool: &PgPool,
    id: i32,
    stock: i32,
    interval: i32,
    current_price: f64,
    baseline_price: f64,
    min_price: f64,
    max_price: f64,
    sensitivity: f64, // FIXED: was sensitvity
) {
    let purchases = query("SELECT COALESCE(SUM(quantity), 0) as total_quantity FROM purchases WHERE product_id = $1 AND completed_at > NOW() - ($2 || ' minutes')::interval")
        .bind(id)
        .bind(interval)  // FIXED: was interval_minutes
        .fetch_one(pool)
        .await
        .unwrap();

    let total_quantity_bought_in_time_period: i64 =
        purchases.try_get("total_quantity").unwrap_or(0);

    // FIXED: Convert to f64 and handle division by zero
    let stock_f64 = stock as f64;
    let quantity_f64 = total_quantity_bought_in_time_period as f64;

    let ratio = if quantity_f64 > 0.0 {
        stock_f64 / quantity_f64
    } else {
        stock_f64
    };

    let new_price = baseline_price * (1.0 + (ratio - 1.0) * sensitivity);

    // ADDED: Clamp between min and max
    let final_price = if new_price < min_price {
        min_price
    } else if new_price > max_price {
        max_price
    } else {
        new_price
    };

    // ADDED: Update database
    query("UPDATE products SET current_price = $1, last_price_update = NOW(), updated_at = NOW() WHERE id = $2")
        .bind(final_price)
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}
