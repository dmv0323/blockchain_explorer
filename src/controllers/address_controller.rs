// src/controllers/address_controller.rs
use rocket::{get, serde::json::Json};
use crate::models::address::Address;

#[get("/addresses/<_id>")]
pub fn get_address(_id: i32) -> Option<Json<Address>> {
    None
}
