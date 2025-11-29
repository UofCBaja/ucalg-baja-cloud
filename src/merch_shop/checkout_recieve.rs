use std::sync::Arc;

use crate::{ArcString, database::Database};
use actix_web::{
    HttpRequest, Responder, post,
    web::{self, Data},
};
use darkicewolf50_actix_setup::log_incoming_w_x;
use serde::{Deserialize, Serialize};
// use serde_json::json;
use tokio::sync::Mutex;
use umya_spreadsheet::{Worksheet, reader, writer};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerInfo {
    pub order_id: Option<ArcString>,
    email: ArcString,
    phone: Option<ArcString>,
    name: ArcString,
    sub_team: Option<ArcString>,
    order_total: f32,
    ship_full_name: Option<ArcString>,
    ship_street_addr: Option<ArcString>,
    ship_unit_number: Option<ArcString>,
    ship_city: Option<ArcString>,
    ship_province: Option<ArcString>,
    ship_country: Option<ArcString>,
    ship_postal_code: Option<ArcString>,
    ship_phone: Option<ArcString>,
    additional_notes: Option<ArcString>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    order_id: Option<ArcString>,
    item_id: ArcString,
    colour: Option<ArcString>,
    size: Option<ArcString>,
    quantity: u8,
    price: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    customer_info: CustomerInfo,
    cart_items: Vec<OrderItem>,
    order_id: Option<ArcString>,
    coupon_code: Option<ArcString>,
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
    mut order_request: web::Json<OrderRequest>,
    req: HttpRequest,
) -> impl Responder {
    log_incoming_w_x("POST", "/shop/recieve_order", req);

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
    order_request.give_uuid();

    let mut book = reader::xlsx::lazy_read(&database.connection.as_ref().unwrap()).unwrap();

    let orders_sheet = book.get_sheet_by_name_mut("orders").unwrap();
    // Finds "newest row"
    let mut order_row_insert = orders_sheet.get_highest_row() + 1;
    // Customer Info Part of incoming order
    let orders = order_request.cart_items.clone();

    let mut order_total: f32 = 0.0;

    // Writes all orders in the vec to the sheet
    for order in orders {
        order_row_insert += database.write_order(&order, orders_sheet, &order_row_insert);
        order_total += order.price;
    }

    order_total *= 1.05;

    database.write_customer(
        &order_request.customer_info,
        &order_total,
        &order_request.coupon_code,
    );

    // save to workbook
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

        // colour
        orders_sheet
            .get_cell_mut(format!("E{}", row_insert))
            .set_value_string(
                order
                    .colour
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );

        // price
        orders_sheet
            .get_cell_mut(format!("F{}", row_insert))
            .set_value_number(order.price);
        1
    }

    pub fn write_customer(
        &self,
        customer_info: &CustomerInfo,
        order_total: &f32,
        coupon: &Option<ArcString>,
    ) {
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

        // Order Total
        customer_sheet
            .get_cell_mut(format!("F{}", customer_row_insert))
            .set_value_number(*order_total);

        // Coupon if input
        customer_sheet
            .get_cell_mut(format!("G{}", customer_row_insert))
            .set_value(coupon.clone().unwrap_or_default().to_string());

        // shipping

        // shipping full name
        customer_sheet
            .get_cell_mut(format!("I{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .ship_full_name
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );

        // shipping street address
        customer_sheet
            .get_cell_mut(format!("J{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .ship_street_addr
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );

        // shipping unit number
        customer_sheet
            .get_cell_mut(format!("K{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .ship_unit_number
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );

        // shipping city
        customer_sheet
            .get_cell_mut(format!("L{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .ship_city
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );

        // shipping provice
        customer_sheet
            .get_cell_mut(format!("M{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .ship_province
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );

        // shipping country
        customer_sheet
            .get_cell_mut(format!("N{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .ship_country
                    .clone()
                    .unwrap_or_else(|| Arc::<str>::from("Canada"))
                    .to_string(),
            );

        // shipping postal code
        customer_sheet
            .get_cell_mut(format!("O{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .ship_postal_code
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );

        // shipping phone number
        customer_sheet
            .get_cell_mut(format!("P{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .ship_phone
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );

        customer_sheet
            .get_cell_mut(format!("Q{}", customer_row_insert))
            .set_value_string(
                customer_info
                    .additional_notes
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            );

        // save to workbook
        writer::xlsx::write(&book, self.connection.as_ref().unwrap().as_path()).unwrap();
    }
}

impl OrderRequest {
    pub fn give_uuid(&mut self) {
        if self.order_id == None {
            let new_uuid: ArcString = Uuid::new_v4().to_string().into();

            self.order_id = Some(new_uuid.clone());

            self.customer_info.order_id = Some(new_uuid.clone());

            for order_item in self.cart_items.iter_mut() {
                order_item.order_id = Some(new_uuid.clone());
            }
        }
    }
}
