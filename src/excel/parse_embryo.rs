use crate::constants::{STORAGE_FILE_PATH, STORAGE_URL_PREFIX};
use crate::dto::dto_excel::EmbryoExcelDto;
use crate::{ERPError, ERPResult};
use std::collections::HashMap;
use umya_spreadsheet::*;

/* 图片,编号,名称,颜色,单位,数量,备注 */
lazy_static! {
    pub static ref J_TO_NAME: HashMap<i32, &'static str> = vec![
        (1, "序号"),
        (2, "编号"),
        (3, "图片"),
        (4, "名称"),
        (5, "电镀&颜色"),
        (6, "数量"),
        (7, "单位"),
        (8, "单价"),
        (9, "金额"),
        (10, "备注")
    ]
    .into_iter()
    .collect();
    pub static ref NONE_NULLABLE_JS: Vec<i32> = vec![2, 4, 5, 6, 7];
}

pub fn parse_embryos(file_path: &str) -> ERPResult<Vec<EmbryoExcelDto>> {
    tracing::info!("fila_path: {file_path}");
    let path = std::path::Path::new(file_path);
    let sheets = reader::xlsx::read(path).unwrap();
    let items_sheet = sheets
        .get_sheet(&0)
        .map_err(|_| ERPError::ExcelError("库存胚sheet未找到".to_string()))?;

    let (cols, rows) = items_sheet.get_highest_column_and_row();

    let mut items = vec![];
    let mut pre: Option<EmbryoExcelDto> = None;

    // 从第2行开始
    for i in 2..rows + 1 {
        print!("row: {}", i);

        let mut cur = EmbryoExcelDto::default();
        if let Some(previous) = pre {
            cur.images = previous.images;
        }

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
                    if j == 3 && !cur.images.is_empty() {
                        continue;
                    }
                    return Err(ERPError::ExcelError(format!(
                        "第{}行的 {} 为空",
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
                    if j == 3 && !cur.images.is_empty() {
                        continue;
                    }
                    return Err(ERPError::ExcelError(format!(
                        "第{}行的 {} 为空",
                        i,
                        J_TO_NAME.get(&(j as i32)).unwrap_or(&"")
                    )));
                }
                continue;
            }

            match j {
                2 => cur.number = cell_value.trim().to_string(),
                4 => cur.name = cell_value.trim().to_string(),
                5 => cur.color = cell_value.trim().to_string(),
                6 => cur.count = cell_value.parse::<i32>().unwrap_or(0),
                7 => cur.unit = cell_value.trim().to_string(),
                8 => cur.cost = (cell_value.parse::<f32>().unwrap_or(0.0) * 100.0) as i32,
                10 => cur.notes = cell_value.trim().to_string(),
                _ => {}
            }
        }

        if images.is_empty() && cur.color.is_empty() && cur.number.is_empty() {
            break;
        }

        if cur.images.is_empty() && images.is_empty() {
            return Err(ERPError::ExcelError(format!("第{}行的 图片 为空", i,)));
        }

        if cur.images.is_empty() {
            cur.images = match images.is_empty() {
                true => vec![],
                false => {
                    let mut image_urls = vec![];
                    for (index, real_goods_image) in images.into_iter().enumerate() {
                        let sku_image_name = format!("{}-{}.png", cur.number, index);
                        let goods_image_path =
                            format!("{}/embryo/{}", STORAGE_FILE_PATH, sku_image_name);
                        real_goods_image.download_image(&goods_image_path);
                        image_urls
                            .push(format!("{}/embryo/{}", STORAGE_URL_PREFIX, sku_image_name));
                    }
                    image_urls
                }
            };
        }

        tracing::info!("rows#{:?}: {:?}", i, cur);
        pre = Some(cur.clone());
        items.push(cur);
    }

    print!("rows: {rows:}: cols: {cols:}");

    Ok(items)
}
