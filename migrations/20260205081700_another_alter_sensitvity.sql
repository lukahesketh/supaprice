-- Add migration script here
ALTER TABLE products
    ALTER COLUMN sensitivity TYPE DOUBLE PRECISION;
