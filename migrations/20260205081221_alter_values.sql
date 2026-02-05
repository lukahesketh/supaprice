-- Add migration script here
-- Check current types
ALTER TABLE products
    ALTER COLUMN baseline_price TYPE DOUBLE PRECISION,
    ALTER COLUMN current_price TYPE DOUBLE PRECISION,
    ALTER COLUMN min_price TYPE DOUBLE PRECISION,
    ALTER COLUMN max_price TYPE DOUBLE PRECISION;
