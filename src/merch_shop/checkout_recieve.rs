use actix_web::{Responder, get, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use umya_spreadsheet::{CellRawValue, reader};
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

    let path = std::path::Path::new("./Database/Orders.xlsx");
    let mut book = reader::xlsx::lazy_read(path).unwrap();
    let mut orders_sheet = book.get_sheet_by_name_mut("orders").unwrap();

    let length_by_order_id = orders_sheet
        .get_cell_value_by_range("A2:A")
        .iter()
        .filter_map(|cell_item| match cell_item.get_raw_value() {
            CellRawValue::Empty => None,
            _ => Some(()),
        })
        .count()
        + 2;

    let cells_to_insert = vec![
        format!("A{}", length_by_order_id),
        format!("B{}", length_by_order_id),
        format!("C{}", length_by_order_id),
        format!("D{}", length_by_order_id),
        format!("E{}", length_by_order_id),
    ];

    web::Json(json!({"body": {"message": cell_to_insert}}))
}
