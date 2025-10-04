use std::fs;

use actix_web::{Responder, get, web};
use serde::{Deserialize, Serialize};
// use serde_json::json;
use serde_yaml_bw;

use crate::log_incoming;

#[derive(Debug, Serialize, Deserialize)]
struct MerchItem {
    name: String,
    category: String,
    sizes_available: Vec<String>,
    price: f32,
    colours: Vec<String>,
    description: String,
    url_image: Vec<String>,
    additional_materials: String,
    material: String,
    cleaning: String,
    size_guide_img_url: String,
}

#[get("/merch")]
pub async fn get_merch() -> impl Responder {
    log_incoming("GET", "/shop/merch");

    let yaml = fs::read_to_string("./Database/merch.yaml").unwrap_or_else(|_| "".to_string());

    let yaml: Vec<MerchItem> = serde_yaml_bw::from_str(&yaml).unwrap_or_else(|_| vec![]);

    web::Json(yaml)
}
