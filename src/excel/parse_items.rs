use crate::constants::{STORAGE_FILE_PATH, STORAGE_URL_PREFIX};
use crate::dto::dto_excel::ItemExcelDto;
use crate::{ERPError, ERPResult};
use umya_spreadsheet::*;

/*
图片（可多张）
名称
颜色
类别（大类)
类别 (小类）
规格
单位
售价
成本
备注（可空）
编号（6位数字，688001，688002...)
*/
pub fn parse_items(file_path: &str) -> ERPResult<Vec<ItemExcelDto>> {
    let path = std::path::Path::new(file_path);
    let mut sheets = reader::xlsx::read(path).unwrap();
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
                // tracing::info!("cell({:?}, {:?}): Null", i, j);
                continue;
            }

            let cell_value = cell.unwrap().get_raw_value().to_string();
            if cell_value.is_empty() {
                // tracing::info!("cell({:?}, {:?}): Empty", i, j);
                continue;
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
                _ => {}
            }
        }

        // if cur.
        if cur.barcode.is_empty() {}

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
