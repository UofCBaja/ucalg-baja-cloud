use std::path::{Path, PathBuf};
use umya_spreadsheet::writer;

pub struct Database {
    pub connection: Option<PathBuf>,
}

impl Database {
    pub fn new() -> Database {
        let xl_path = Path::new("./Database/Merch.xlsx");

        let mut database = Database {
            connection: Some(xl_path.into()),
        };

        if !Path::exists(&xl_path) {
            let _ = database.database_initialize_xl();

            return database;
        }

        database
    }

    pub fn check_path_xl(&mut self) -> Result<bool, String> {
        let xl_path = Path::new("./Database/Merch.xlsx");

        if Path::exists(&xl_path) {
            self.connection = Some(xl_path.into());
            return Ok(true);
        } else {
            // Recreating Path
            self.connection = Some(xl_path.into());
            match self.database_initialize_xl() {
                Ok(_) => return Ok(true),
                Err(e) => return Err(e),
            }
        }
    }

    pub fn database_initialize_xl(&mut self) -> Result<(), String> {
        println!("Creating New Sheet");

        let mut book = umya_spreadsheet::new_file_empty_worksheet();

        let order_sheet = match book.new_sheet("orders") {
            Ok(p) => p,
            Err(_) => return Err("Cannot create orders sheet".to_string()),
        };

        order_sheet.get_cell_mut("A1").set_value("Order Id");
        order_sheet.get_cell_mut("A1").set_value("Order Id");
        order_sheet.get_cell_mut("B1").set_value("Email");
        order_sheet.get_cell_mut("C1").set_value("Phone");
        order_sheet.get_cell_mut("D1").set_value("Name");
        order_sheet.get_cell_mut("E1").set_value("Subteam");

        let custmer_sheet = match book.new_sheet("customer_info") {
            Ok(p) => p,
            Err(_) => return Err("Cannot create customer info sheet".to_string()),
        };

        custmer_sheet.get_cell_mut("A1").set_value("Order Id");
        custmer_sheet.get_cell_mut("B1").set_value("Email");
        custmer_sheet.get_cell_mut("C1").set_value("Phone");
        custmer_sheet.get_cell_mut("D1").set_value("Name");
        custmer_sheet.get_cell_mut("E1").set_value("Subteam");
        custmer_sheet.get_cell_mut("F1").set_value("Order Total");
        custmer_sheet.get_cell_mut("G1").set_value("Coupon Code");
        custmer_sheet
            .get_cell_mut("H1")
            .set_value("Shipping Details");
        custmer_sheet.get_cell_mut("I1").set_value("Full Name");
        custmer_sheet.get_cell_mut("J1").set_value("Street Address");
        custmer_sheet.get_cell_mut("K1").set_value("Unit Number");
        custmer_sheet.get_cell_mut("L1").set_value("City");
        custmer_sheet.get_cell_mut("M1").set_value("Province");
        custmer_sheet
            .get_cell_mut("N1")
            .set_value("Country (Default Canada)");
        custmer_sheet.get_cell_mut("O1").set_value("Postal Code");
        custmer_sheet
            .get_cell_mut("P1")
            .set_value("Phone Number (shipping)");

        match writer::xlsx::write(&book, self.connection.as_ref().unwrap().clone().as_path()) {
            Ok(_) => (),
            Err(_) => return Err("Cannot create xl sheet".to_string()),
        };

        Ok(())
    }
}
