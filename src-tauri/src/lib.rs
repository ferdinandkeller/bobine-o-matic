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
    delivery_duration: String,
    security_coeff: String,
    order_frequency: String,
    average_consumption: String,
    std_dev: String,
    stock_quantity: String,
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
    let data2: Vec<Vec<String>> = data
        .iter()
        .map(|row| {
            vec![
                row.reference.clone(),
                row.designation.clone(),
                row.product_brand.clone(),
                row.delivery_duration.clone(),
                row.security_coeff.clone(),
                row.order_frequency.clone(),
                row.average_consumption.clone(),
                row.std_dev.clone(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                row.stock_quantity.clone(),
            ]
        })
        .collect();

    // add headers
    let headers = vec![
        "Référence".to_string(),
        "Désignation".to_string(),
        "Marque".to_string(),
        "Durée de Livraison (jours)".to_string(),
        "Coefficient de Sécurité (99.9%)".to_string(),
        "Fréquence de Commande (jours)".to_string(),
        "Consommation Moyenne / jour".to_string(),
        "Ecart-type".to_string(),
        "Stock Minimum".to_string(),
        "Stock de Sécurité".to_string(),
        "Seuil de Commande".to_string(),
        "Stock Actuel".to_string(),
        "Delta de Stock".to_string(),
        "Quantité à Commander".to_string(),
        "Croissance".to_string(),
    ];
    let mut rows = vec![headers];
    rows.extend(data2);

    let row_count = rows.len();

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    for (i, row) in rows.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            worksheet.write(i as u32, j as u16, cell).unwrap();
        }
    }

    // excel formulas to compute the values
    // for row in range(1, len(out_df) + 1):
    //     # min_stock = average * (delivery_duration + order_frequency)
    //     worksheet.write(row, 7, f"=F{row + 1}*(C{row + 1}+E{row + 1})")
    //     # security_stock = std_dev * security_coeff * sqrt(delivery_duration + order_frequency)
    //     worksheet.write(
    //         row, 8, f"=D{row + 1}*G{row + 1}*SQRT(C{row + 1}+E{row + 1})"
    //     )
    //     # threshold_stock = ceil(min_stock + security_stock)
    //     worksheet.write(row, 9, f"=ROUNDUP(H{row + 1}+I{row + 1},0)")
    //     # threshold_stock - stock
    //     worksheet.write(row, 11, f"=J{row + 1}-K{row + 1}")
    //     # order_quantity = ceil(delta_from_target + min_stock) if delta_from_target >= 0 else 0
    //     worksheet.write(
    //         row, 12, f"=IF(L{row + 1}>=0,ROUNDUP(H{row + 1}+L{row + 1},0),0)"
    //     )

    // write formulas
    for i in 0..(row_count - 1) {
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
        worksheet.write_formula(row - 1, 13, f13).unwrap();
        worksheet.write_formula(row - 1, 14, "20%").unwrap();
    }

    workbook.save(filename).unwrap();
}
