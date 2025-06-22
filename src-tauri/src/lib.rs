use rust_xlsxwriter::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, excel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn excel(content: String, filename: String) {
    let rows: Vec<Vec<&str>> = content
        .lines()
        .map(|line| line.split(',').collect())
        .collect();
    let row_count = rows.len();

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // write data
    for (i, row) in rows.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            worksheet.write(i as u32, j as u16, *cell).unwrap();
        }
    }

    // write formulas
    for i in 0..(row_count - 1) {
        let row = i as u32 + 2;

        let f7 = Formula::new(format!("=F{row}*(C{row} + E{row})"));
        worksheet.write_formula(row - 1, 7, f7).unwrap();
        let f8 = Formula::new(format!("=D{row}*G{row}*SQRT(C{row}+E{row})"));
        worksheet.write_formula(row - 1, 8, f8).unwrap();
        let f9 = Formula::new(format!("=ROUNDUP(H{row}+I{row},0)"));
        worksheet.write_formula(row - 1, 9, f9).unwrap();
        let f11 = Formula::new(format!("=J{row}-K{row}"));
        worksheet.write_formula(row - 1, 11, f11).unwrap();
        let f12 = Formula::new(format!("=IF(L{row}>=0,ROUNDUP(H{row}+L{row},0),0)"));
        worksheet.write_formula(row - 1, 12, f12).unwrap();
    }

    workbook.save(filename).unwrap();
}
