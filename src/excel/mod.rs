// use crate::common::datetime::parse_date;
// use crate::model::order::{OrderInfo, OrderItemExcel};
// use umya_spreadsheet::Worksheet;

pub mod parse_embryo;
mod parse_import_orders;
pub mod parse_items;
pub mod parse_orders;

// pub trait OrderExcelHandler {
//     fn parse_order_info(&self, sheet: &Worksheet) -> OrderInfo {
//         let mut order_info = OrderInfo::default();
//         let (cols, _rows) = sheet.get_highest_column_and_row();
//         for i in 1..6 {
//             for j in 1..cols + 1 {
//                 let cell = sheet.get_cell((j, i));
//                 if cell.is_none() {
//                     continue;
//                 }
//
//                 let cell_value = cell.unwrap().get_raw_value().to_string();
//                 if cell_value.is_empty() {
//                     continue;
//                 }
//
//                 if cell_value.contains("客户") {
//                     let mut customer_no = cell_value.strip_prefix("客户:").unwrap_or("");
//                     if customer_no.is_empty() {
//                         customer_no = cell_value.strip_prefix("客户：").unwrap_or("");
//                     }
//                     order_info.customer_no = customer_no.to_owned();
//                 }
//
//                 if cell_value.contains("供应商") {
//                     let mut customer_no = cell_value.strip_prefix("供应商:").unwrap_or("");
//                     if customer_no.is_empty() {
//                         customer_no = cell_value.strip_prefix("供应商：").unwrap_or("");
//                     }
//                     order_info.customer_no = customer_no.to_owned();
//                 }
//
//                 if cell_value.contains("单号") {
//                     let mut order_no = cell_value.strip_prefix("单号:").unwrap_or("");
//                     if order_no.is_empty() {
//                         order_no = cell_value.strip_prefix("单号：").unwrap_or("");
//                     }
//                     order_info.order_no = order_no.to_owned();
//                 }
//
//                 if cell_value.contains("订货日期") {
//                     let mut order_date = cell_value.strip_prefix("订货日期:").unwrap_or("");
//                     if order_date.is_empty() {
//                         order_date = cell_value.strip_prefix("订货日期：").unwrap_or("");
//                     }
//                     order_info.order_date = parse_date(order_date).unwrap();
//                 }
//
//                 if cell_value.contains("交货日期") {
//                     let mut delivery_date = cell_value.strip_prefix("交货日期:").unwrap_or("");
//                     if delivery_date.is_empty() {
//                         delivery_date = cell_value.strip_prefix("交货日期：").unwrap_or("");
//                     }
//                     order_info.delivery_date = parse_date(delivery_date);
//
//                     if delivery_date.contains("返单") {
//                         order_info.is_return_order = true;
//                     }
//                     if delivery_date.contains("加急") {
//                         order_info.is_urgent = true;
//                     }
//                 }
//             }
//         }
//
//         order_info
//     }
//
//     fn parse_order_items(&self) -> Vec<OrderItemExcel>;
//
//     fn save_order(&self);
// }
