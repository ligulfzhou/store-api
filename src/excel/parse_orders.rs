use crate::common::datetime::{get_cur_date_str, parse_date_with_regex};
use crate::constants::{STORAGE_FILE_PATH, STORAGE_URL_PREFIX};
use crate::dto::dto_excel::OrderExcelDto;
use crate::{ERPError, ERPResult};
use chrono::NaiveDate;
use std::collections::HashMap;
use std::fs;
use tracing_subscriber::fmt::format;
use umya_spreadsheet::*;

lazy_static! {
    pub static ref J_TO_NAME: HashMap<i32, &'static str> = vec![
        (1, "序号"),
        (2, "编号"),
        (3, "图片"),
        (4, "尺寸"),
        (5, "名称"),
        (6, "颜色"),
        (7, "数量"),
        (8, "单位"),
        (9, "售价"),
        (10, "金额"),
        (11, "备注")
    ]
    .into_iter()
    .collect();
    pub static ref NONE_NULLABLE_JS: Vec<i32> = vec![2, 4, 5, 6, 7, 8, 9, 10];
}

#[derive(Debug, Default)]
pub struct OrderInfo {
    pub order_date: NaiveDate,
    pub delivery_date: NaiveDate,
}

pub async fn parse_order_info(file_path: &str) -> ERPResult<OrderInfo> {
    let mut order_info = OrderInfo::default();

    tracing::info!("file_path: {file_path}");
    let path = std::path::Path::new(file_path);
    let sheets = reader::xlsx::read(path).unwrap();
    let items_sheet = sheets
        .get_sheet(&0)
        .map_err(|_| ERPError::ExcelError("商品sheet未找到".to_string()))?;

    let (cols, _) = items_sheet.get_highest_column_and_row();

    let row = 2;
    for j in 1..cols + 1 {
        let cell = items_sheet.get_cell((j, row));
        if cell.is_none() {
            continue;
        }

        let cell_value = cell.unwrap().get_raw_value().to_string();
        if cell_value.is_empty() {
            continue;
        }

        if cell_value.contains("订单") {
            let naive_order_date = parse_date_with_regex(&cell_value)
                .ok_or(ERPError::ExcelError("出货日期未找到".to_string()))?;
            order_info.order_date = naive_order_date;
        }

        if cell_value.contains("出货") {
            let naive_order_date = parse_date_with_regex(&cell_value)
                .ok_or(ERPError::ExcelError("出货日期未找到".to_string()))?;
            order_info.delivery_date = naive_order_date;
        }
    }

    Ok(order_info)
}

pub async fn parse_order(file_path: &str) -> ERPResult<Vec<OrderExcelDto>> {
    tracing::info!("file_path: {file_path}");
    let path = std::path::Path::new(file_path);
    let sheets = reader::xlsx::read(path).unwrap();
    let items_sheet = sheets
        .get_sheet(&0)
        .map_err(|_| ERPError::ExcelError("商品sheet未找到".to_string()))?;

    let (cols, rows) = items_sheet.get_highest_column_and_row();

    let mut items = vec![];
    let mut pre: Option<OrderExcelDto> = None;

    let now_str = get_cur_date_str();

    let mut index_to_images: HashMap<i32, Vec<String>> = HashMap::new();
    // // 从第4行开始
    for i in 4..rows + 1 {
        print!("row: {}", i);

        let mut cur = OrderExcelDto::default();
        if let Some(previous) = pre {
            // 编号
            cur.number = previous.number;
        }

        let mut images: Vec<&Image> = vec![];
        for j in 1..cols + 1 {
            if j == 3 {
                images = items_sheet.get_images((j, i));
                tracing::info!("images: {:?}", images.len());
                continue;
            }
            let cell = items_sheet.get_cell((j, i));
            if cell.is_none() {
                continue;
            }

            let cell_value = cell.unwrap().get_raw_value().to_string();
            if cell_value.is_empty() {
                continue;
            }

            match j {
                1 => cur.index = cell_value.trim().parse::<i32>().unwrap_or(0),
                2 => cur.number = cell_value.trim().to_string(),
                4 => cur.size = cell_value.trim().to_string(),
                5 => cur.name = cell_value.trim().to_string(),
                6 => cur.color = cell_value.trim().to_string().to_ascii_uppercase(),
                7 => cur.count = (cell_value.parse::<f32>().unwrap_or(0.0) * 10.0) as i32,
                8 => cur.unit = cell_value.trim().to_string(),
                9 => cur.price = (cell_value.parse::<f32>().unwrap_or(0.0) * 100.0) as i32,
                10 => cur.total = (cell_value.parse::<f32>().unwrap_or(0.0) * 100.0) as i32,
                11 => cur.notes = cell_value.trim().to_string(),
                _ => {}
            }
        }

        let mut image_urls = vec![];
        if !images.is_empty() {
            for (_, real_goods_image) in images.into_iter().enumerate() {
                let image_name = format!("{}/{}-{}.png", &now_str, cur.index, cur.number);
                let image_path = format!("{}/order/{}", STORAGE_FILE_PATH, image_name);
                fs::create_dir_all(format!("{}/order/{}", STORAGE_FILE_PATH, &now_str))?;
                real_goods_image.download_image(&image_path);
                image_urls.push(format!("{}/order/{}", STORAGE_URL_PREFIX, image_name));
            }
        }
        if !image_urls.is_empty() {
            index_to_images.insert(cur.index, image_urls);
        }

        if cur.count == 0 || cur.price == 0 {
            break;
        }

        tracing::info!("rows#{:?}: {:?}", i, cur);
        pre = Some(cur.clone());
        items.push(cur);
    }

    print!("rows: {rows:}: cols: {cols:}");

    for (index, item) in items.iter().enumerate() {
        if item.count == 0 || item.price == 0 {
            let name = match item.count == 0 {
                true => "数量",
                _ => "单价",
            };
            return Err(ERPError::ExcelError(format!(
                "第{}行的 {} 为空/0",
                index + 4,
                name
            )));
        }
    }

    // insert images to items
    let empty_image_urls: Vec<String> = vec![];
    for item in items.iter_mut() {
        let image_urls = index_to_images
            .get(&item.index)
            .unwrap_or(&empty_image_urls);
        item.images = image_urls.clone();
    }

    Ok(items)
}
