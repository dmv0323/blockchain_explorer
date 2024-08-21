// src/config/config.rs

pub struct Config {
  pub database_url: String,
  pub port: u16,
}

impl Config {
  pub fn new() -> Self {
      Self {
          database_url: "postgres://localhost/blockchain_explorer".into(),
          port: 8000,
      }
  }
}
