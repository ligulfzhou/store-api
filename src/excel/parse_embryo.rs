use crate::constants::{STORAGE_FILE_PATH, STORAGE_URL_PREFIX};
use crate::dto::dto_excel::EmbryoExcelDto;
use crate::{ERPError, ERPResult};
use std::collections::HashMap;
use umya_spreadsheet::*;

/* 图片(可多张)编号，图片，名称，颜色，单位，数量 */

lazy_static! {
    pub static ref J_TO_NAME: HashMap<i32, &'static str> = vec![
        (1, "图片"),
        (2, "编号"),
        (3, "名称"),
        (4, "颜色"),
        (5, "单位"),
        (6, "数量")
    ]
    .into_iter()
    .collect();
    pub static ref NONE_NULLABLE_JS: Vec<i32> = vec![2, 3, 4, 5, 6];
}

pub fn parse_embryos(file_path: &str) -> ERPResult<Vec<EmbryoExcelDto>> {
    let path = std::path::Path::new(file_path);
    let sheets = reader::xlsx::read(path).unwrap();
    let items_sheet = sheets
        .get_sheet(&0)
        .map_err(|_| ERPError::ExcelError("库存胚sheet未找到".to_string()))?;

    let (cols, rows) = items_sheet.get_highest_column_and_row();

    let mut items = vec![];

    // 从第2行开始
    for i in 2..rows + 1 {
        print!("row: {}", i);

        let mut cur = EmbryoExcelDto::default();

        let mut images: Vec<&Image> = vec![];
        for j in 1..cols + 1 {
            if j == 1 {
                images = items_sheet.get_images((j, i));
                print!("images count: {}", images.len());
            }

            let cell = items_sheet.get_cell((j, i));
            if cell.is_none() {
                if NONE_NULLABLE_JS.contains(&(j as i32)) {
                    return Err(ERPError::ExcelError(format!(
                        "第{}行的 {} 为空",
                        i,
                        J_TO_NAME.get(&(j as i32)).unwrap_or(&"")
                    )));
                }
            }

            let cell_value = cell.unwrap().get_raw_value().to_string();
            if cell_value.is_empty() {
                if NONE_NULLABLE_JS.contains(&(j as i32)) {
                    return Err(ERPError::ExcelError(format!(
                        "第{}行的 {} 为空",
                        i,
                        J_TO_NAME.get(&(j as i32)).unwrap_or(&"")
                    )));
                }
                continue;
            }

            if j == 6 {
                tracing::info!("cell_value: {}", cell_value);

                let count = cell_value.parse::<i32>().unwrap_or(0);
                if count == 0 {
                    return Err(ERPError::ExcelError(format!("第{}行的数量栏为空/0", i)));
                }
            }

            match j {
                2 => cur.number = cell_value.trim().to_string(),
                3 => cur.name = cell_value.trim().to_string(),
                4 => cur.color = cell_value.trim().to_string(),
                5 => cur.unit = cell_value.trim().to_string(),
                6 => cur.count = cell_value.parse::<i32>().unwrap_or(0),
                _ => {}
            }
        }

        if images.is_empty() {
            return Err(ERPError::ExcelError(format!("第{}行的 图片 为空", i,)));
        }

        let mut image_urls = vec![];
        if !images.is_empty() {
            for (index, real_goods_image) in images.into_iter().enumerate() {
                let sku_image_name = format!("{}-{}.png", cur.number, index);
                let goods_image_path = format!("{}/embryo/{}", STORAGE_FILE_PATH, sku_image_name);
                real_goods_image.download_image(&goods_image_path);
                image_urls.push(format!("{}/embryo/{}", STORAGE_URL_PREFIX, sku_image_name));
            }
        }
        cur.images = image_urls;

        tracing::info!("rows#{:?}: {:?}", i, cur);
        items.push(cur);
    }

    print!("rows: {rows:}: cols: {cols:}");

    Ok(items)
}
