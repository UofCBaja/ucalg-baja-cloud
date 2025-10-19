use actix_web::{Responder, get, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use umya_spreadsheet::{CellRawValue, reader, writer};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderItem {
    order_id: Option<String>,
    item_id: String,
    size: Option<String>,
    quantity: u8,
    price: f32,
}

#[get("/recieve_order")]
pub async fn recieve_order() -> impl Responder {
    log_incoming("GET", "/shop/recieve_order");

    let fake_order = OrderItem {
        order_id: Some(Uuid::new_v4().to_string()),
        item_id: "HERO-2020 HOODIES".to_string(),
        size: Some("M".to_string()),
        quantity: 1,
        price: 36.01,
    };

    let path = std::path::Path::new("./Database/Orders.xlsx");
    let mut book = reader::xlsx::lazy_read(path).unwrap();

    let orders_sheet = book.get_sheet_by_name_mut("orders").unwrap();

    // accomplished by using order Id, finds the next row to insert into
    let row_to_insert = orders_sheet
        .get_cell_value_by_range("A2:A")
        .iter()
        .filter_map(|cell_item| match cell_item.get_raw_value() {
            CellRawValue::Empty => None,
            _ => Some(()),
        })
        .count()
        + 2;

    let order = fake_order;

    // order id
    orders_sheet
        .get_cell_mut(format!("A{}", row_to_insert))
        .set_value(order.order_id.clone().unwrap_or_default());

    // item id
    orders_sheet
        .get_cell_mut(format!("B{}", row_to_insert))
        .set_value(order.item_id.clone());

    // size
    orders_sheet
        .get_cell_mut(format!("C{}", row_to_insert))
        .set_value(order.size.clone().unwrap_or_else(|| "".to_string()));

    // quantity
    orders_sheet
        .get_cell_mut(format!("D{}", row_to_insert))
        .set_value_number(order.quantity);

    // price
    orders_sheet
        .get_cell_mut(format!("E{}", row_to_insert))
        .set_value_number(order.price);

    // --- save workbook ---
    writer::xlsx::write(&book, path).unwrap();

    web::Json(json!(
        {
            "body": {
                "message": {
                    "order_id": order.order_id.unwrap_or_else(|| "".to_string()),
                    "item_id": order.item_id,
                    "size": order.size.unwrap_or_else(|| "".to_string()),
                    "quantity": order.quantity,
                    "price": order.price
                }
            }
        }
    ))
}
