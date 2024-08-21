use super::*;
use crate::models::transaction::Transaction;
use crate::mempool::{Mempool, SharedMempool};
use std::sync::Arc;

#[test]
fn test_add_and_get_transaction() {
    let mempool = SharedMempool::new(Mempool::new());
    let tx = Transaction { id: "tx1".to_string(), ..Default::default() };

    mempool.add_transaction(tx.clone());

    let retrieved_tx = mempool.get_transaction(&tx.id);
    assert_eq!(retrieved_tx, Some(tx));
}

#[test]
fn test_remove_transaction() {
    let mempool = SharedMempool::new(Mempool::new());
    let tx = Transaction { id: "tx1".to_string(), ..Default::default() };

    mempool.add_transaction(tx.clone());
    mempool.remove_transaction(&tx.id);

    let retrieved_tx = mempool.get_transaction(&tx.id);
    assert_eq!(retrieved_tx, None);
}
