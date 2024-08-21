// src/controllers/transaction_controller.rs
use rocket::{get, post, State, serde::json::Json};
use crate::models::transaction::Transaction;
use crate::services::transaction_service::TransactionService;
use crate::mempool::mempool::SharedMempool;

#[post("/transactions", format = "json", data = "<transaction>")]
pub fn add_transaction(transaction: Json<Transaction>, _mempool: &State<SharedMempool>) {
    TransactionService::create_transaction(transaction.into_inner()).unwrap();
}

#[get("/transactions/<_id>")]
pub fn get_transaction(_id: i32) -> Option<Json<Transaction>> {
    None
}
