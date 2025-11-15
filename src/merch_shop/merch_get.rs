use std::fs;

use actix_web::{Responder, get, web};
use darkicewolf50_actix_setup::log_incoming;
use serde::{Deserialize, Serialize};
// use serde_json::json;
use serde_yaml_bw;

use crate::{ArcString, ArcVec};

#[derive(Debug, Serialize, Deserialize)]
struct MerchItem {
    name: ArcString,
    category: ArcString,
    sizes_available: ArcVec<ArcString>,
    price: f32,
    colours: Vec<ArcString>,
    description: ArcString,
    url_image: ArcVec<ArcString>,
    additional_materials: ArcString,
    material: ArcString,
    cleaning: ArcString,
    size_guide_img_url: ArcString,
}

#[get("/merch")]
pub async fn get_merch() -> impl Responder {
    log_incoming("GET", "/shop/merch");

    let yaml = fs::read_to_string("./Database/merch.yaml").unwrap_or_else(|_| "".to_string());

    let yaml: ArcVec<MerchItem> = serde_yaml_bw::from_str(&yaml)
        .unwrap_or_else(|_| vec![])
        .into();

    web::Json(yaml)
}
