use actix_web::{HttpResponse, Responder, get, post, web};
use darkicewolf50_actix_setup::log_incoming;
use serde::{Deserialize, Serialize};
use serde_json::json;
use umya_spreadsheet::{reader, writer};
use uuid::Uuid;

use crate::xl_init::{self, init_xl_doc};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CustomerInfo {
    order_id: Option<String>,
    email: String,
    phone: Option<String>,
    name: String,
    sub_team: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderItem {
    order_id: Option<String>,
    item_id: String,
    size: Option<String>,
    quantity: u8,
    price: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    customer_info: CustomerInfo,
    cart_items: Vec<OrderItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderSuccess {
    success: bool,
    failure: Option<String>,
    testing: Option<OrderRequest>,
}

#[post("/recieve_order")]
pub async fn recieve_order(order_request: web::Json<OrderRequest>) -> impl Responder {
    log_incoming("POST", "/shop/recieve_order");

    // println!("{:?}", order_request);

    // let fake_order_id = Uuid::new_v4().to_string();

    // let fake_order = OrderRequest {
    //     customer_info: OrderUserInfo {
    //         order_id: Some(fake_order_id.clone()),
    //         uofc_email: "brock.tomlinson@ucalgary.ca".to_string(),
    //         phone: "2509466196".to_string(),
    //         name: "Brock".to_string(),
    //         sub_team: Some("Software".to_string()),
    //     },
    //     cart_items: vec![
    //         OrderItem {
    //             order_id: Some(fake_order_id.clone()),
    //             item_id: "HERO-2020 HOODIES".to_string(),
    //             size: Some("M".to_string()),
    //             quantity: 1,
    //             price: 36.01,
    //         },
    //         OrderItem {
    //             order_id: Some(fake_order_id.clone()),
    //             item_id: "HERO-2020 HOODIES".to_string(),
    //             size: Some("XL".to_string()),
    //             quantity: 1,
    //             price: 36.01,
    //         },
    //     ],
    // };

    // println!("{:?}", fake_order);
    let path = match init_xl_doc() {
        Ok(p) => p,
        Err(e) => {
            return web::Json(OrderSuccess {
                success: false,
                failure: Some(e),
                testing: None,
            });
        }
    };
    let mut book = reader::xlsx::lazy_read(&path).unwrap();

    let customer_sheet = book.get_sheet_by_name_mut("customer_info").unwrap();
    // Finds "newest row"
    let customer_row_insert = customer_sheet.get_highest_row() + 1;
    // Customer Info Part of incoming order
    let customer_info = order_request.customer_info.clone();

    // order id
    customer_sheet
        .get_cell_mut(format!("A{}", customer_row_insert))
        .set_value(customer_info.order_id.clone().unwrap_or_default());

    // email
    customer_sheet
        .get_cell_mut(format!("B{}", customer_row_insert))
        .set_value(customer_info.email.clone());
    // phone
    customer_sheet
        .get_cell_mut(format!("C{}", customer_row_insert))
        .set_value_string(customer_info.phone.clone().unwrap_or_default());
    // name
    customer_sheet
        .get_cell_mut(format!("D{}", customer_row_insert))
        .set_value(customer_info.name.clone());
    // subteam
    customer_sheet
        .get_cell_mut(format!("E{}", customer_row_insert))
        .set_value(customer_info.sub_team.clone().unwrap_or_default());

    let orders_sheet = book.get_sheet_by_name_mut("orders").unwrap();
    // Finds "newest row"
    let mut order_row_insert = orders_sheet.get_highest_row() + 1;
    // Customer Info Part of incoming order
    let orders = order_request.cart_items.clone();

    // Writes all orders in the vec to the sheet
    for order in &orders {
        // order id
        orders_sheet
            .get_cell_mut(format!("A{}", order_row_insert))
            .set_value(order.order_id.clone().unwrap_or_default());

        // item id
        orders_sheet
            .get_cell_mut(format!("B{}", order_row_insert))
            .set_value(order.item_id.clone());

        // size
        orders_sheet
            .get_cell_mut(format!("C{}", order_row_insert))
            .set_value(order.size.clone().unwrap_or_else(|| "".to_string()));

        // quantity
        orders_sheet
            .get_cell_mut(format!("D{}", order_row_insert))
            .set_value_number(order.quantity);

        // price
        orders_sheet
            .get_cell_mut(format!("E{}", order_row_insert))
            .set_value_number(order.price);

        order_row_insert += 1;
    }

    // --- save workbook ---
    writer::xlsx::write(&book, path).unwrap();

    web::Json(OrderSuccess {
        success: true,
        failure: None,
        testing: Some(order_request.into_inner()),
    })

    // web::Json(json!(
    //     {
    //         "body": {
    //             "message": [
    //                 {
    //                     "order_id": orders.get(0).unwrap().order_id,
    //                     "item_id": orders.get(0).unwrap().item_id,
    //                     "size": orders.get(0).unwrap().size,
    //                     "quantity": orders.get(0).unwrap().quantity,
    //                     "price": orders.get(0).unwrap().price
    //                 },
    //                 {
    //                     "order_id": orders.get(1).unwrap().order_id,
    //                     "item_id": orders.get(1).unwrap().item_id,
    //                     "size": orders.get(1).unwrap().size,
    //                     "quantity": orders.get(1).unwrap().quantity,
    //                     "price": orders.get(1).unwrap().price
    //                 }
    //             ]
    //         }
    //     }
    // ))
}
