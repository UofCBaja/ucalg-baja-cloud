use crate::{ArcString, ArcVec, database::Database};
use actix_web::{
    Responder, post,
    web::{self, Data},
};
use darkicewolf50_actix_setup::log_incoming;
use serde::{Deserialize, Serialize};
// use serde_json::json;
use tokio::sync::Mutex;
use umya_spreadsheet::{Worksheet, reader, writer};
// use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerInfo {
    order_id: Option<ArcString>,
    email: ArcString,
    phone: Option<ArcString>,
    name: ArcString,
    sub_team: Option<ArcString>,
    shipping_details: ArcString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    order_id: Option<ArcString>,
    item_id: ArcString,
    size: Option<ArcString>,
    quantity: u8,
    price: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    customer_info: CustomerInfo,
    cart_items: ArcVec<OrderItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderSuccess {
    success: bool,
    failure: Option<ArcString>,
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
                failure: Some(e.into()),
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
    for order in orders.as_ref().iter() {
        order_row_insert += database.write_order(order, orders_sheet, &order_row_insert);
    }

    // --- save workbook ---
    writer::xlsx::write(&book, &database.connection.as_ref().unwrap()).unwrap();

    web::Json(OrderSuccess {
        success: true,
        failure: None,
        testing: Some(order_request.into_inner()),
    })
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
            .set_value(
                order
                    .order_id
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );

        // item id
        orders_sheet
            .get_cell_mut(format!("B{}", row_insert))
            .set_value_string(order.item_id.as_ref().to_string());

        // size
        orders_sheet
            .get_cell_mut(format!("C{}", row_insert))
            .set_value(
                order
                    .size
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );

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
            .set_value(
                customer_info
                    .order_id
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );

        // email
        customer_sheet
            .get_cell_mut(format!("B{}", customer_row_insert))
            .set_value(customer_info.email.as_ref().to_string());
        // phone
        customer_sheet
            .get_cell_mut(format!("C{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .phone
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );
        // name
        customer_sheet
            .get_cell_mut(format!("D{}", customer_row_insert))
            .set_value_string(customer_info.name.to_string());
        // subteam
        customer_sheet
            .get_cell_mut(format!("E{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .sub_team
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );

        customer_sheet
            .get_cell_mut(format!("F{}", customer_row_insert))
            .set_value_string(customer_info.shipping_details.as_ref().to_string());

        writer::xlsx::write(&book, self.connection.as_ref().unwrap().as_path()).unwrap();
    }
}
