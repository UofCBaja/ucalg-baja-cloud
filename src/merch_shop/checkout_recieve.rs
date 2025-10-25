use std::collections::HashMap;

use actix_web::{Responder, get, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use umya_spreadsheet::{CellRawValue, reader, writer};
use uuid::Uuid;

use crate::log_incoming;

#[derive(Debug, Serialize, Deserialize)]
struct OrderUserInfo {
    order_id: Option<String>,
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

#[derive(Deserialize)]
pub struct OrderRequest {
    pub user: OrderUserInfo,
    pub items: Vec<OrderItem>,
}

#[get("/recieve_order/{order_request}")]
pub async fn recieve_order(order_request: web::Json<OrderRequest>) -> impl Responder {
    log_incoming("GET", "/shop/recieve_order");

    let fake_order_id = Uuid::new_v4().to_string();

    let fake_order = vec![
        OrderItem {
            order_id: Some(fake_order_id.clone()),
            item_id: "HERO-2020 HOODIES".to_string(),
            size: Some("M".to_string()),
            quantity: 1,
            price: 36.01,
        },
        OrderItem {
            order_id: Some(fake_order_id.clone()),
            item_id: "HERO-2020 HOODIES".to_string(),
            size: Some("XL".to_string()),
            quantity: 1,
            price: 36.01,
        },
    ];

    let path = std::path::Path::new("./Database/Orders.xlsx");
    let mut book = reader::xlsx::lazy_read(path).unwrap();

    let orders_sheet = book.get_sheet_by_name_mut("orders").unwrap();

    let mut row_to_insert = orders_sheet.get_highest_row() + 1;
    // accomplished by using order Id, finds the next row to insert into
    // let mut row_to_insert = orders_sheet
    //     .get_cell_value_by_range("A2:A")
    //     .iter()
    //     .filter_map(|cell_item| match cell_item.get_raw_value() {
    //         CellRawValue::Empty => None,
    //         _ => Some(()),
    //     })
    //     .count()
    //     + 2;

    let orders = fake_order;

    for order in &orders {
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

        row_to_insert += 1;
    }
    // --- save workbook ---
    writer::xlsx::write(&book, path).unwrap();

    web::Json(json!(
        {
            "body": {
                "message": [
                    {
                        "order_id": orders.get(0).unwrap().order_id,
                        "item_id": orders.get(0).unwrap().item_id,
                        "size": orders.get(0).unwrap().size,
                        "quantity": orders.get(0).unwrap().quantity,
                        "price": orders.get(0).unwrap().price
                    },
                    {
                        "order_id": orders.get(1).unwrap().order_id,
                        "item_id": orders.get(1).unwrap().item_id,
                        "size": orders.get(1).unwrap().size,
                        "quantity": orders.get(1).unwrap().quantity,
                        "price": orders.get(1).unwrap().price
                    }
                ]
            }
        }
    ))
}
