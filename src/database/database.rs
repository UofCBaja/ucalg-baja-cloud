use std::path::{Path, PathBuf};
use umya_spreadsheet::writer;

pub struct Database {
    connection: Option<PathBuf>,
}

impl Database {
    pub fn new() -> Database {
        let xl_path = Path::new("./Database/Merch.xlsx");

        Database {
            connection: Some(xl_path.into()),
        }
        .check_connection_created_xl()
        .unwrap()
    }
    pub fn check_connection_created_xl(&mut self) -> Result<(), String> {
        let xl_path = Path::new("./Database/Merch.xlsx");

        if Path::exists(&xl_path) {
            self.connection = Some(xl_path.into());
            return Ok(());
        }

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
        custmer_sheet.get_cell_mut("A1").set_value("Order Id");
        custmer_sheet.get_cell_mut("B1").set_value("Email");
        custmer_sheet.get_cell_mut("C1").set_value("Phone");
        custmer_sheet.get_cell_mut("D1").set_value("Name");
        custmer_sheet.get_cell_mut("E1").set_value("Subteam");

        match writer::xlsx::write(&book, xl_path) {
            Ok(_) => (),
            Err(_) => return Err("Cannot create xl sheet".to_string()),
        };

        self.connection = Some(xl_path.into());
        Ok(())
    }
}
