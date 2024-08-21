#[macro_use] extern crate rocket;

mod models;
mod services;
mod controllers;
mod db;
mod mempool;
mod utils;
mod websocket;

use rocket::{Rocket, Build, routes, catchers, fs::FileServer};
use rocket::serde::json::Json;
use rocket::State;
use tera::Tera;
use std::sync::Arc;
use rocket::tokio::sync::broadcast;

use crate::controllers::{transaction_controller, address_controller};
use crate::mempool::mempool::{Mempool, SharedMempool};
use crate::websocket::{blockstream, start_blockstream};

#[get("/")]
async fn index(tera: &rocket::State<Tera>) -> Result<rocket::response::content::RawHtml<String>, rocket::http::Status> {
    let context = tera::Context::new();
    match tera.render("index.html", &context) {
        Ok(rendered) => Ok(rocket::response::content::RawHtml(rendered)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[catch(404)]
fn not_found() -> rocket::response::content::RawHtml<String> {
    rocket::response::content::RawHtml("<h1>Página não encontrada</h1><p>Desculpe, mas a página que você está procurando não existe.</p>".to_string())
}

#[launch]
fn rocket() -> Rocket<Build> {
    let mempool = SharedMempool::new(Mempool::new());
    let blockstream_tx = rocket::tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(start_blockstream());

    rocket::build()
        .manage(mempool)
        .manage(blockstream_tx)
        .manage(Tera::new("templates/**/*").unwrap())
        .mount("/", routes![index])
        .mount("/transactions", routes![transaction_controller::add_transaction, transaction_controller::get_transaction])
        .mount("/addresses", routes![address_controller::get_address])
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![blockstream])
        .register("/", catchers![not_found])
}
