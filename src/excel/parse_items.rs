use crate::common::items::calculate_barcode;
use crate::constants::{STORAGE_FILE_PATH, STORAGE_URL_PREFIX};
use crate::dto::dto_excel::ItemExcelDto;
use crate::{ERPError, ERPResult};
use std::collections::HashMap;
use umya_spreadsheet::*;

/*图片(可多张) 名称	颜色	产品大类	产品小类(可空) 编号	条码(可空) 规格	单位	售价	成本	备注(可空) 数量(6位数字，688001，688002...)*/
/* 名称	颜色	产品大类 编号 单位	售价	成本 数量 */

/* 序号 编号 图片 规格 名称 产品大类 产品小类 电镀&颜色 条码 数量 单位 单价 售价 金额 备注 */
lazy_static! {
    pub static ref J_TO_NAME: HashMap<i32, &'static str> = vec![
        (1, "序号"),
        (2, "编号"),
        (3, "图片"),
        (4, "规格"),
        (5, "名称"),
        (6, "产品大类"),
        (7, "产品小类"),
        (8, "电镀&颜色"),
        (9, "条码"),
        (10, "数量"),
        (11, "单位"),
        (12, "售价"),
        (13, "成本"),
        (14, "金额"),
        (15, "备注")
    ]
    .into_iter()
    .collect();
    pub static ref NONE_NULLABLE_JS: Vec<i32> = vec![2, 5, 6, 8, 10, 11, 12, 13];
}

pub fn parse_items(
    file_path: &str,
    color_to_value: HashMap<String, i32>,
) -> ERPResult<Vec<ItemExcelDto>> {
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
            // 图片在第3列
            if j == 3 {
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
                } else {
                    continue;
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
                2 => cur.number = cell_value.trim().to_string(),
                4 => cur.size = cell_value.trim().to_string(),
                5 => cur.name = cell_value.trim().to_string(),
                6 => cur.cates1 = cell_value.trim().to_string(),
                7 => cur.cates2 = cell_value.trim().to_string(),
                8 => cur.color = cell_value.trim().to_string().to_ascii_uppercase(),
                9 => cur.barcode = cell_value.trim().to_string(),
                10 => cur.count = cell_value.parse::<i32>().unwrap_or(0),
                11 => cur.unit = cell_value.trim().to_string(),
                12 => cur.cost = (cell_value.parse::<f32>().unwrap_or(0.0) * 100.0) as i32,
                13 => cur.price = (cell_value.parse::<f32>().unwrap_or(0.0) * 100.0) as i32,
                15 => cur.notes = cell_value.trim().to_string(),
                _ => {}
            }
        }

        if images.is_empty() {
            return Err(ERPError::ExcelError(format!("第{}行的 图片 为空", i)));
        }

        if cur.barcode.is_empty() {
            if !color_to_value.contains_key(&cur.color) {
                return Err(ERPError::ExcelError(format!(
                    "第{}行的 颜色{} 没有在后台配置对应数值",
                    i, cur.color
                )));
            }

            let color_value = color_to_value.get(&cur.color).unwrap_or(&0);
            cur.barcode = calculate_barcode(&cur.number, *color_value, cur.price);
        }

        // let mut image_urls = vec![];
        // if !images.is_empty() {
        //     for (index, real_goods_image) in images.into_iter().enumerate() {
        //         let sku_image_name = format!("{}-{}.png", cur.barcode, index);
        //         let goods_image_path = format!("{}/sku/{}", STORAGE_FILE_PATH, sku_image_name);
        //         real_goods_image.download_image(&goods_image_path);
        //         image_urls.push(format!("{}/sku/{}", STORAGE_URL_PREFIX, sku_image_name));
        //     }
        // }
        // cur.images = image_urls;

        cur.images = match images.is_empty() {
            true => vec![],
            _ => {
                let mut tmp = vec![];
                for (index, real_goods_image) in images.into_iter().enumerate() {
                    let sku_image_name = format!("{}-{}.png", cur.barcode, index);
                    let goods_image_path = format!("{}/sku/{}", STORAGE_FILE_PATH, sku_image_name);
                    real_goods_image.download_image(&goods_image_path);
                    tmp.push(format!("{}/sku/{}", STORAGE_URL_PREFIX, sku_image_name));
                }

                tmp
            }
        };

        tracing::info!("rows#{:?}: {:?}", i, cur);
        items.push(cur);
    }

    print!("rows: {rows:}: cols: {cols:}");

    Ok(items)
}
