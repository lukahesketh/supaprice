use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct CreateStoreStruct {
    pub domain: String,
    pub business_name: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Store {
    pub id: i32,
    pub domain: String,
    pub business_name: String,
}
