#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use reqwest;
use std::collections::HashMap;

#[get("/")]
async fn index() -> &'static str {
    "Welcome to Hathor Blockchain Explorer!"
}

#[get("/block/<block_id>")]
async fn block_details(block_id: &str) -> Json<HashMap<String, serde_json::Value>> {
    let url = format!("https://node1.mainnet.hathor.network/v1a/block/{}", block_id);
    let response = reqwest::get(&url).await.unwrap().json::<HashMap<String, serde_json::Value>>().await.unwrap();
    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, block_details])
}
