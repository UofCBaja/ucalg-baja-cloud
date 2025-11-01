use std::path::{Path, PathBuf};
use umya_spreadsheet::writer;

pub fn init_xl_doc() -> Result<PathBuf, String> {
    let xl_path = Path::new("./Database/Merch.xlsx");

    if Path::exists(&xl_path) {
        return Ok(xl_path.into());
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

    Ok(xl_path.into())
}
