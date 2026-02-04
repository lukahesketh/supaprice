# Dynamic Pricing Backend System - Codebase Summary

## Overview
This is a Rust-based backend service that implements a dynamic pricing system for e-commerce stores. It automatically adjusts product prices based on demand (purchase velocity) and stock levels using a background worker process.

## Tech Stack
- **Language**: Rust (Edition 2024)
- **Web Framework**: Axum 0.8.8
- **Database**: PostgreSQL with SQLx 0.8.6
- **Runtime**: Tokio (async)
- **Additional**: CORS support, environment variables via dotenvy

## Architecture

### Main Application (`src/main.rs`)
- Connects to PostgreSQL database using connection string from `DATABASE_URL` environment variable
- Spawns a background worker task for price calculation
- Serves HTTP API on `0.0.0.0:3069`
- Uses connection pooling for database access

### Data Models (`src/data_structures/`)

**Store** (`store.rs`):
- `domain`: String - Store's domain name
- `business_name`: String - Store's business name

**Product** (`product.rs`):
- `store_id`: i32 - Foreign key to stores table
- `external_id`: String - External product identifier
- `baseline_price`: f64 - Base price for calculations
- `current_price`: f64 - Current dynamic price
- `min_price`: f64 - Minimum allowed price
- `max_price`: f64 - Maximum allowed price
- `sensitivity`: f64 - Price adjustment sensitivity (0-1)
- `stock`: i32 - Current stock level
- `update_interval_minutes`: i32 - How often to recalculate price

**Order** (`order.rs`):
- `product_id`: i32 - Product being purchased
- `quantity`: i32 - Quantity purchased

### API Endpoints (`src/routers/`)

**POST /init_store**
- Creates a new store
- Body: `{ "domain": "string", "business_name": "string" }`
- Returns: Created store object

**POST /init_product**
- Creates a new product for a store
- Body: Product object with all fields
- Returns: `{ "id": number }` - The created product ID

**POST /init_order**
- Records a purchase/order
- Body: `{ "product_id": number, "quantity": number }`
- Triggers price recalculation on next cycle

**GET /get_price/{id}**
- Retrieves current price for a product
- Returns: `{ "id": number, "current_price": number }`

### Background Worker (`calculate_price.rs`)

**Price Calculation Algorithm**:
1. Runs every 150 seconds
2. Finds products due for price update (based on `update_interval_minutes`)
3. For each product:
   - Calculates total quantity purchased in the last interval period
   - Computes ratio: `stock / quantity_bought`
   - Applies formula: `new_price = baseline_price * (1 + (ratio - 1) * sensitivity)`
   - Clamps result between `min_price` and `max_price`
   - Updates database with new price and timestamp

**Key Logic**:
- High demand (low ratio) → price increases
- Low demand (high ratio) → price decreases
- Sensitivity controls how aggressively prices change
- Prices are bounded by min/max constraints

## Database Schema (`migrations/20260201160441_initial_setup.sql`)

**stores table**:
- `id` (SERIAL PRIMARY KEY)
- `domain` (VARCHAR UNIQUE)
- `business_name` (VARCHAR)
- `created_at` (TIMESTAMP)

**products table**:
- `id` (SERIAL PRIMARY KEY)
- `store_id` (FK to stores)
- `external_id` (VARCHAR)
- `baseline_price`, `current_price`, `min_price`, `max_price` (DECIMAL)
- `sensitivity` (DECIMAL 0-1)
- `stock` (INTEGER)
- `update_interval_minutes` (INTEGER, must be multiple of 5, between 5-180)
- `last_price_update`, `created_at`, `updated_at` (TIMESTAMPS)
- Unique constraint on (store_id, external_id)
- Various CHECK constraints for data validation

**purchases table**:
- `id` (SERIAL PRIMARY KEY)
- `product_id` (FK to products)
- `completed_at` (TIMESTAMP)
- `quantity` (INTEGER)
- Indexed on (product_id, completed_at) for efficient queries

## Configuration
- CORS enabled for `http://localhost:3000`
- Allows GET and POST methods
- Environment variables loaded from `.env` file
- Database connection via `DATABASE_URL`

## Use Case
This system is designed for e-commerce platforms that want to implement dynamic pricing based on real-time demand. Stores can set price boundaries and sensitivity levels, and the system automatically adjusts prices to optimize revenue based on purchase patterns and inventory levels.
