use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderStruct {
    pub product_id: i32, // this needs to be the ID that the init_product gave us
    pub quantity: i32,
}
