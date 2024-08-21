// src/models/address.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub id: i32,
    pub address: String,
}
