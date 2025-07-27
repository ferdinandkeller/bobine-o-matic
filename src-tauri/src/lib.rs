use calamine::{self, Reader};
use regex::Regex;
use rust_xlsxwriter::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![excel, excel_1, excel_2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DataRow {
    date: String,
    quantity: f64,
    product_code: String,
    product_name: String,
    product_brand: String,
    subfamily_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DataRow2 {
    product_code: String,
    valuated_quantity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProcessedDataRow {
    reference: String,
    designation: String,
    product_brand: String,
    subfamily_name: String,
    delivery_duration: f64,
    security_coeff: f64,
    order_frequency: f64,
    average_consumption: f64,
    std_dev: f64,
    stock_quantity: f64,
}

fn open_file_1(file_path: String) -> Vec<DataRow> {
    // open the Excel file
    let mut workbook: calamine::Xlsx<_> = calamine::open_workbook(file_path).unwrap();
    let worksheets = workbook.worksheets();
    let (_name, worksheet) = worksheets.get(0).unwrap();

    // load the data from the first worksheet
    let content = worksheet
        .rows()
        .skip(12)
        .map(|row| {
            row.iter()
                .map(|cell| cell.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    // find which columns we need
    let columns: Vec<String> = content[0].iter().map(|c| c.trim().to_lowercase()).collect();
    let date_index = columns.iter().position(|c| c == "date").unwrap();
    let quantity_index = columns.iter().position(|c| c == "quantity").unwrap();
    let product_code_index = columns.iter().position(|c| c == "product code").unwrap();
    let produce_name_index = columns.iter().position(|c| c == "product name").unwrap();
    let product_brand_index = columns.iter().position(|c| c == "brand group").unwrap();
    let product_subfamily_index = columns
        .iter()
        .position(|c| c == "local subfamily name")
        .unwrap();

    let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    // parse into DataRow structs
    let data: Vec<DataRow> = content
        .iter()
        .skip(1)
        .filter(|row| {
            // make sure the date matches the regex
            if !date_regex.is_match(&row[date_index]) {
                return false;
            }
            return true;
        })
        .map(|row| DataRow {
            // make sure the date is in the correct format
            date: row[date_index].clone(),
            quantity: row[quantity_index].parse().unwrap_or(0.0),
            product_code: row[product_code_index].clone(),
            product_name: row[produce_name_index].clone(),
            product_brand: row[product_brand_index].clone(),
            subfamily_name: row[product_subfamily_index].clone(),
        })
        .filter(|row| row.quantity > 0.0)
        .collect();

    data
}

fn open_file_2(file_path: String) -> Vec<DataRow2> {
    // open the Excel file
    let mut workbook: calamine::Xlsx<_> = calamine::open_workbook(file_path).unwrap();
    let worksheets = workbook.worksheets();
    let (_name, worksheet) = worksheets.get(2).unwrap();

    // load the data from the first worksheet
    let content = worksheet
        .rows()
        .skip(3)
        .map(|row| {
            row.iter()
                .map(|cell| cell.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    // find which columns we need
    let columns: Vec<String> = content[0].iter().map(|c| c.trim().to_lowercase()).collect();
    let product_code_index = columns.iter().position(|c| c == "product code").unwrap();
    let quantity_index = columns
        .iter()
        .position(|c| c == "valuated quantity")
        .unwrap();

    // parse into DataRow structs
    let data: Vec<DataRow2> = content
        .iter()
        .skip(1)
        .map(|row| DataRow2 {
            // make sure the date is in the correct format,
            product_code: row[product_code_index].clone(),
            valuated_quantity: row[quantity_index].replace(',', ".").parse().unwrap_or(0.0),
        })
        .filter(|row| row.valuated_quantity > 0.0)
        .collect();

    data
}

#[tauri::command]
fn excel_1(file_path_1: String) -> String {
    let data_1 = open_file_1(file_path_1);
    serde_json::to_string(&data_1).unwrap()
}

#[tauri::command]
fn excel_2(file_path_2: String) -> String {
    let data_2 = open_file_2(file_path_2);
    serde_json::to_string(&data_2).unwrap()
}

#[tauri::command]
fn excel(content: String, filename: String) {
    let data: Vec<ProcessedDataRow> = serde_json::from_str(&content).unwrap();

    // create a new Excel workbook and worksheet
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // add headers
    let format_bold = Format::new().set_bold();
    worksheet
        .write_string_with_format(0, 0, "Référence", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 1, "Désignation", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 2, "Marque", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 3, "Durée de Livraison (jours)", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 4, "Coefficient de Sécurité", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 5, "Fréquence de Commande (jours)", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 6, "Consommation Moyenne / jour", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 7, "Ecart-type", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 8, "Stock Minimum", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 9, "Stock de Sécurité", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 10, "Seuil de Commande", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 11, "Stock Actuel", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 12, "Delta de Stock", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 13, "Quantité à Commander", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 14, "Croissance", &format_bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 15, "Local SubFamily Name", &format_bold)
        .unwrap();

    // add data rows
    for (i, row) in data.iter().enumerate() {
        let row_index = i as u32 + 1;
        worksheet
            .write_string(row_index, 0, &row.reference)
            .unwrap();
        worksheet
            .write_string(row_index, 1, &row.designation)
            .unwrap();
        worksheet
            .write_string(row_index, 2, &row.product_brand)
            .unwrap();
        worksheet
            .write_number(row_index, 3, row.delivery_duration)
            .unwrap();
        worksheet
            .write_number(row_index, 4, row.security_coeff)
            .unwrap();
        worksheet
            .write_number(row_index, 5, row.order_frequency)
            .unwrap();
        worksheet
            .write_number(row_index, 6, row.average_consumption)
            .unwrap();
        worksheet.write_number(row_index, 7, row.std_dev).unwrap();
        worksheet
            .write_number(row_index, 11, row.stock_quantity)
            .unwrap();
        worksheet
            .write_string(row_index, 15, &row.subfamily_name)
            .unwrap();
    }

    // write formulas
    let row_count = data.len();
    for i in 0..row_count {
        let row = i as u32 + 2;
        let f8 = Formula::new(format!("=G{row}*(D{row} + F{row})*(1+O{row})"));
        worksheet.write_formula(row - 1, 8, f8).unwrap();
        let f9 = Formula::new(format!("=E{row}*H{row}*SQRT(D{row}+F{row})"));
        worksheet.write_formula(row - 1, 9, f9).unwrap();
        let f10 = Formula::new(format!("=ROUNDUP(I{row}+J{row},0)"));
        worksheet.write_formula(row - 1, 10, f10).unwrap();
        let f12 = Formula::new(format!("=K{row}-L{row}"));
        worksheet.write_formula(row - 1, 12, f12).unwrap();
        let f13 = Formula::new(format!("=IF(M{row}>=0,ROUNDUP(I{row}+M{row},0),0)"));
        worksheet
            .write_formula_with_format(row - 1, 13, f13, &format_bold)
            .unwrap();
        worksheet.write_formula(row - 1, 14, "20%").unwrap();
    }

    workbook.save(filename).unwrap();
}
