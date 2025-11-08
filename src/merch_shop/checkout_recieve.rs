use crate::database::Database;
use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, Data},
};
use darkicewolf50_actix_setup::log_incoming;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;
use umya_spreadsheet::{Spreadsheet, Worksheet, reader, writer};
use uuid::Uuid;

use crate::xl_init::init_xl_doc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerInfo {
    order_id: Option<String>,
    email: String,
    phone: Option<String>,
    name: String,
    sub_team: Option<String>,
    shipping_details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
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
pub async fn recieve_order(
    data_state: Data<Mutex<Database>>,
    order_request: web::Json<OrderRequest>,
) -> impl Responder {
    log_incoming("POST", "/shop/recieve_order");

    let mut database = data_state.lock().await;

    match database.check_path_xl() {
        Ok(_) => (),
        Err(e) => {
            return web::Json(OrderSuccess {
                success: false,
                failure: Some(e),
                testing: None,
            });
        }
    }
    database.write_customer(&order_request.customer_info);

    let mut book = reader::xlsx::lazy_read(&database.connection.as_ref().unwrap()).unwrap();

    let orders_sheet = book.get_sheet_by_name_mut("orders").unwrap();
    // Finds "newest row"
    let mut order_row_insert = orders_sheet.get_highest_row() + 1;
    // Customer Info Part of incoming order
    let orders = order_request.cart_items.clone();

    // Writes all orders in the vec to the sheet
    for order in &orders {
        order_row_insert += database.write_order(order, orders_sheet, &order_row_insert);
    }

    // --- save workbook ---
    writer::xlsx::write(&book, &database.connection.as_ref().unwrap()).unwrap();

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

impl Database {
    pub fn write_order(
        &self,
        order: &OrderItem,
        orders_sheet: &mut Worksheet,
        row_insert: &u32,
    ) -> u32 {
        // order id
        orders_sheet
            .get_cell_mut(format!("A{}", row_insert))
            .set_value(order.order_id.clone().unwrap_or_default());

        // item id
        orders_sheet
            .get_cell_mut(format!("B{}", row_insert))
            .set_value(order.item_id.clone());

        // size
        orders_sheet
            .get_cell_mut(format!("C{}", row_insert))
            .set_value(order.size.clone().unwrap_or_else(|| "".to_string()));

        // quantity
        orders_sheet
            .get_cell_mut(format!("D{}", row_insert))
            .set_value_number(order.quantity);

        // price
        orders_sheet
            .get_cell_mut(format!("E{}", row_insert))
            .set_value_number(order.price);

        1
    }

    pub fn write_customer(&self, customer_info: &CustomerInfo) {
        let mut book =
            reader::xlsx::lazy_read(self.connection.as_ref().unwrap().as_path()).unwrap();

        let customer_sheet = book.get_sheet_by_name_mut("customer_info").unwrap();
        // Finds "newest row"
        let customer_row_insert = customer_sheet.get_highest_row() + 1;

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

        customer_sheet
            .get_cell_mut(format!("F{}", customer_row_insert))
            .set_value(customer_info.shipping_details.clone());

        writer::xlsx::write(&book, self.connection.as_ref().unwrap().as_path()).unwrap();
    }
}
