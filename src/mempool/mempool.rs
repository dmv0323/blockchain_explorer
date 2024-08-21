// src/mempool/mempool.rs
pub struct Mempool;

impl Mempool {
    pub fn new() -> Self {
        Mempool
    }
}

pub struct SharedMempool;

impl SharedMempool {
    pub fn new(_mempool: Mempool) -> Self {
        SharedMempool
    }
}
