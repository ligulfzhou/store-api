use crate::common::items::calculate_barcode;
use crate::constants::{STORAGE_FILE_PATH, STORAGE_URL_PREFIX};
use crate::dto::dto_excel::ItemExcelDto;
use crate::{ERPError, ERPResult};
use std::collections::HashMap;
use umya_spreadsheet::*;

/*图片(可多张) 名称	颜色	产品大类	产品小类(可空) 编号	条码(可空) 规格	单位	售价	成本	备注(可空) 数量(6位数字，688001，688002...)*/

lazy_static! {
    pub static ref J_TO_NAME: HashMap<i32, &'static str> = vec![
        (1, "图片"),
        (2, "名称"),
        (3, "颜色"),
        (4, "产品大类"),
        (5, "产品小类"),
        (6, "编号"),
        (7, "条码"),
        (8, "规格"),
        (9, "单位"),
        (10, "售价"),
        (11, "成本"),
        (12, "备注"),
        (13, "数量")
    ]
    .into_iter()
    .collect();
    pub static ref NONE_NULLABLE_JS: Vec<i32> = vec![2, 3, 4, 6, 9, 10, 11, 13];
}

pub fn parse_items(file_path: &str) -> ERPResult<Vec<ItemExcelDto>> {
    let path = std::path::Path::new(file_path);
    let sheets = reader::xlsx::read(path).unwrap();
    let items_sheet = sheets
        .get_sheet(&0)
        .map_err(|_| ERPError::ExcelError("商品sheet未找到".to_string()))?;

    let (cols, rows) = items_sheet.get_highest_column_and_row();

    let mut items = vec![];

    // 从第2行开始
    for i in 2..rows + 1 {
        print!("row: {}", i);

        let mut cur = ItemExcelDto::default();

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
                        "第{}行的 {} 为空/0",
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

            if j == 13 {
                tracing::info!("cell_value: {}", cell_value);

                let count = cell_value.parse::<i32>().unwrap_or(0);
                if count == 0 {
                    return Err(ERPError::ExcelError(format!("第{}行的数量栏为空/0", i)));
                }
            }

            match j {
                // 1 => cur.name = cell_value.trim().to_string(),
                2 => cur.name = cell_value.trim().to_string(),
                3 => cur.color = cell_value.trim().to_string(),
                4 => cur.cates1 = cell_value.trim().to_string(),
                5 => cur.cates2 = cell_value.trim().to_string(),
                6 => cur.number = cell_value.trim().to_string(),
                7 => cur.barcode = cell_value.trim().to_string(),
                8 => cur.size = cell_value.trim().to_string(),
                9 => cur.unit = cell_value.trim().to_string(),
                10 => cur.price = (cell_value.parse::<f32>().unwrap_or(0.0) * 100.0) as i32,
                11 => cur.cost = (cell_value.parse::<f32>().unwrap_or(0.0) * 100.0) as i32,
                12 => cur.notes = cell_value.trim().to_string(),
                13 => cur.count = cell_value.parse::<i32>().unwrap_or(0),
                _ => {}
            }
        }

        if images.is_empty() {
            return Err(ERPError::ExcelError(format!("第{}行的 图片 为空", i,)));
        }

        if cur.barcode.is_empty() {
            // todo: color=> value
            cur.barcode = calculate_barcode(&cur.number, 1, cur.price);
        }

        let mut image_urls = vec![];
        if !images.is_empty() {
            for (index, real_goods_image) in images.into_iter().enumerate() {
                let sku_image_name = format!("{}-{}.png", cur.barcode, index);
                let goods_image_path = format!("{}/sku/{}", STORAGE_FILE_PATH, sku_image_name);
                real_goods_image.download_image(&goods_image_path);
                image_urls.push(format!("{}/sku/{}", STORAGE_URL_PREFIX, sku_image_name));
            }
        }
        cur.images = image_urls;

        tracing::info!("rows#{:?}: {:?}", i, cur);
        items.push(cur);
    }

    print!("rows: {rows:}: cols: {cols:}");

    Ok(items)
}
