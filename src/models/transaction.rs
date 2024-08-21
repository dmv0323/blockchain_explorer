// src/models/transaction.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub hash: String,
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
}
