// src/services/transaction_service.rs
use crate::models::transaction::Transaction;

pub struct TransactionService;

impl TransactionService {
    pub fn create_transaction(_transaction: Transaction) -> Result<(), String> {

        Ok(())
    }
}
