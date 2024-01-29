use crate::common::items::calculate_barcode;
use crate::common::list::pickup_most_common_string;
use crate::constants::{STORAGE_FILE_PATH, STORAGE_URL_PREFIX};
use crate::dto::dto_excel::ItemExcelDto;
use crate::service::settings_service::SettingsServiceTrait;
use crate::state::excel_state::ExcelState;
use crate::{ERPError, ERPResult};
use itertools::Itertools;
use std::collections::HashMap;
use umya_spreadsheet::*;

/* 图片(可多张) 名称	颜色	产品大类	产品小类(可空) 编号	条码(可空) 规格	单位	售价	成本	备注(可空) 数量(6位数字，688001，688002...)*/
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
        (12, "成本"),
        (13, "售价"),
        (14, "金额"),
        (15, "备注")
    ]
    .into_iter()
    .collect();
    pub static ref NONE_NULLABLE_JS: Vec<i32> = vec![2, 5, 8, 10, 11, 13];
}

pub async fn parse_items<'a>(
    state: &ExcelState,
    file_path: &str,
    color_to_value: HashMap<String, i32>,
) -> ERPResult<Vec<ItemExcelDto<'a>>> {
    let mut new_color_to_value = color_to_value.clone();

    tracing::info!("file_path: {file_path}");
    let path = std::path::Path::new(file_path);
    let sheets = reader::xlsx::read(path).unwrap();
    let items_sheet = sheets
        .get_sheet(&0)
        .map_err(|_| ERPError::ExcelError("商品sheet未找到".to_string()))?;

    let (cols, rows) = items_sheet.get_highest_column_and_row();

    let mut items = vec![];
    let mut pre: Option<ItemExcelDto> = None;

    // get colors first
    let mut new_colors_to_empty_value = HashMap::new();
    for i in 7..rows + 1 {
        let cell = items_sheet.get_cell((8, i));
        if cell.is_none() {
            continue;
        }
        let cell_value = cell.unwrap().get_raw_value().to_string();
        if cell_value.is_empty() {
            continue;
        }

        let color = cell_value.trim().to_string().to_ascii_uppercase();

        // tracing::info!(
        //     "color: {:?}, contains: {:?}",
        //     color,
        //     new_color_to_value.contains_key(&color)
        // );
        if !new_color_to_value.contains_key(&color) {
            new_colors_to_empty_value.entry(color).or_insert("");
        }
    }
    // tracing::info!("new_colors_to_empty_value: {:?}", new_colors_to_empty_value);

    if !new_colors_to_empty_value.is_empty() {
        let new_colors = new_colors_to_empty_value
            .into_iter()
            .map(|item| item.0)
            .collect::<Vec<_>>();

        if !new_colors.is_empty() {
            let c_to_v = state
                .settings_service
                .add_multiple_color_to_value(new_colors)
                .await?;

            c_to_v.into_iter().for_each(|(c, v)| {
                new_color_to_value.insert(c, v);
            });
        }
    }

    // 从第2行开始
    for i in 7..rows + 1 {
        print!("row: {}", i);

        let mut cur = ItemExcelDto::default();
        if let Some(previous) = pre {
            // cur.images = previous.images;
            cur.index = previous.index;
            cur.number = previous.number;
            cur.size = previous.size;
            cur.name = previous.name;
        }

        // let mut images: Vec<&Image> = vec![];
        for j in 1..cols + 1 {
            // 图片在第3列
            if j == 3 {
                cur.raw_excel_images = items_sheet.get_images((j, i)).clone();
                print!("images count: {}", cur.raw_excel_images.len());
            }

            let cell = items_sheet.get_cell((j, i));
            if cell.is_none() {
                if NONE_NULLABLE_JS.contains(&(j as i32)) {
                    if j == 2 && !cur.number.is_empty() || j == 5 && !cur.name.is_empty() {
                        continue;
                    }

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
                    if j == 2 && !cur.number.is_empty() || j == 5 && !cur.name.is_empty() {
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
                1 => cur.index = cell_value.trim().parse::<i32>().unwrap_or(0),
                2 => cur.number = cell_value.trim().to_string(),
                4 => cur.size = cell_value.trim().to_string(),
                5 => cur.name = cell_value.trim().to_string(),
                6 => cur.cates1 = cell_value.trim().to_string(),
                7 => cur.cates2 = cell_value.trim().to_string(),
                8 => cur.color = cell_value.trim().to_string().to_ascii_uppercase(),
                9 => cur.barcode = cell_value.trim().to_string(),
                10 => cur.count = (cell_value.parse::<f32>().unwrap_or(0.0) * 10.0) as i32,
                11 => cur.unit = cell_value.trim().to_string(),
                12 => cur.cost = (cell_value.parse::<f32>().unwrap_or(0.0) * 100.0) as i32,
                13 => cur.price = (cell_value.parse::<f32>().unwrap_or(0.0) * 100.0) as i32,
                15 => cur.notes = cell_value.trim().to_string(),
                _ => {}
            }
        }

        if cur.color.is_empty() && cur.number.is_empty() && cur.cost == 0 && cur.price == 0 {
            break;
        }

        if cur.barcode.is_empty() {
            let color_value = new_color_to_value.get(&cur.color).unwrap_or(&0);
            cur.barcode = calculate_barcode(&cur.number, *color_value, cur.price / 5);
        }

        // if cur.images.is_empty() && images.is_empty() {
        //     return Err(ERPError::ExcelError(format!("第{}行的 图片 为空", i)));
        // }

        // if !images.is_empty() {
        //     cur.images = match images.is_empty() {
        //         true => vec![],
        //         _ => {
        //             let mut tmp = vec![];
        //             for (index, real_goods_image) in images.into_iter().enumerate() {
        //                 let sku_image_name = format!("{}-{}.png", cur.barcode, index);
        //                 let goods_image_path =
        //                     format!("{}/sku/{}", STORAGE_FILE_PATH, sku_image_name);
        //                 real_goods_image.download_image(&goods_image_path);
        //                 tmp.push(format!("{}/sku/{}", STORAGE_URL_PREFIX, sku_image_name));
        //             }
        //
        //             tmp
        //         }
        //     };
        // }

        tracing::info!(
            "index: {}, rows#{:?}: {:?}, {:?}, {:?}，{:?},{:?}",
            cur.index,
            i,
            cur.number,
            cur.color,
            cur.price,
            new_color_to_value.get(&cur.color).unwrap_or(&0),
            cur.barcode
        );
        pre = Some(cur.clone());
        items.push(cur);
    }

    print!("rows: {rows:}: cols: {cols:}");

    let mut index_to_items = HashMap::new();
    items.clone().into_iter().for_each(|item| {
        index_to_items
            .entry(item.index)
            .or_insert(vec![])
            .push(item);
    });

    let mut fixed_items = vec![];

    for (index, index_items) in index_to_items.into_iter().sorted_by_key(|x| x.0) {
        let (cate1, cate2) = get_cate1_and_cate2_from_items(index, &index_items)?;
        let images = get_images_from_items(index, &index_items)?;

        let index_items_clone = index_items
            .clone()
            .into_iter()
            .map(|item| ItemExcelDto {
                cates1: cate1.clone(),
                cates2: cate2.clone(),
                images: images.clone(),
                raw_excel_images: vec![],
                ..item
            })
            .collect::<Vec<_>>();

        fixed_items.extend(index_items_clone);
    }

    Ok(fixed_items)
}

fn get_cate1_and_cate2_from_items(
    index: i32,
    items: &[ItemExcelDto<'_>],
) -> ERPResult<(String, String)> {
    let cate1s = items.iter().map(|item| &item.cates1).collect::<Vec<_>>();
    let cate2s = items.iter().map(|item| &item.cates2).collect::<Vec<_>>();

    let cate1 = pickup_most_common_string(&cate1s);
    let cate2 = pickup_most_common_string(&cate2s);

    if cate1.is_empty() || cate2.is_empty() {
        return Err(ERPError::NotFound(format!("序号{}内未找到大小类", index)));
    }

    Ok((cate1, cate2))
}

fn get_images_from_items(index: i32, items: &[ItemExcelDto<'_>]) -> ERPResult<Vec<String>> {
    for item in items.iter() {
        if !item.raw_excel_images.is_empty() {
            let mut images = vec![];
            for (image_index, real_goods_image) in item.raw_excel_images.iter().enumerate() {
                let sku_image_name = format!("{}-{}.png", item.barcode, image_index);
                let goods_image_path = format!("{}/sku/{}", STORAGE_FILE_PATH, sku_image_name);
                real_goods_image.download_image(&goods_image_path);
                images.push(format!("{}/sku/{}", STORAGE_URL_PREFIX, sku_image_name));
            }

            return Ok(images);
        }
    }

    Err(ERPError::NotFound(format!("序号 {} 没找到图片", index)))
}
