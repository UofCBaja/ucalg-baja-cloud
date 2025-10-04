use actix_web::{Responder, get, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::log_incoming;

#[derive(Debug, Serialize, Deserialize)]
struct OrderUserInfo {
    order_id: Option<(u64, u64)>,
    uofc_email: String,
    phone: String,
    name: String,
    sub_team: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderItem {
    order_id: Option<(u64, u64)>,
    item_id: String,
    size: Option<String>,
    quantity: u8,
    price: f32,
}

#[get("/recieve_order")]
pub async fn recieve_order() -> impl Responder {
    log_incoming("GET", "/shop/recieve_order");
    web::Json(json!({"body": {"message": "this is a temp response"}}))
}
