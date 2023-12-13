pub fn calculate_barcode(number: &str, color: i32, price: i32) -> String {
    format!("{}{:02}{:04}", number, color, price)
}
