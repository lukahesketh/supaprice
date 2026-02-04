-- Add migration script here
CREATE TABLE stores (
    id SERIAL PRIMARY KEY,
    domain VARCHAR(255) UNIQUE NOT NULL,
    business_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id) ON DELETE CASCADE,
    external_id VARCHAR(255) NOT NULL,
    baseline_price DECIMAL(10, 2) NOT NULL,
    current_price DECIMAL(10, 2) NOT NULL,
    min_price DECIMAL(10, 2) NOT NULL,
    max_price DECIMAL(10, 2) NOT NULL,
    sensitivity DECIMAL(4, 3) NOT NULL,
    stock INTEGER NOT NULL,
    update_interval_minutes INTEGER NOT NULL DEFAULT 5,
    last_price_update TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(store_id, external_id),
    CHECK (min_price <= max_price),
    CHECK (min_price >= 0),
    CHECK (sensitivity >= 0 AND sensitivity <= 1),
    CHECK (stock >= 0),
    CHECK (update_interval_minutes > 0),
    CHECK (update_interval_minutes % 5 = 0 AND update_interval_minutes >= 5 AND update_interval_minutes <= 180)
);

CREATE TABLE purchases (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    completed_at TIMESTAMP NOT NULL DEFAULT NOW(),
    quantity INTEGER NOT NULL DEFAULT 1,
    CHECK (quantity > 0)
);

CREATE INDEX idx_purchases_product_time ON purchases(product_id, completed_at);
CREATE INDEX idx_products_next_update ON products(last_price_update);
