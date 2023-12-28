use crate::state::excel_state::ExcelState;
use crate::ERPResult;
use std::collections::HashMap;

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
        (9, "单价"),
        (10, "金额"),
        (11, "备注")
    ]
    .into_iter()
    .collect();
    pub static ref NONE_NULLABLE_JS: Vec<i32> = vec![2, 4, 5, 6, 8, 10, 11, 12, 13];
}

pub async fn parse_orders(
    state: &ExcelState,
    file_path: &str,
    color_to_value: HashMap<String, i32>,
) -> ERPResult<()> {
    // ) -> ERPResult<Vec<ItemExcelDto>> {
    todo!()

    // let mut new_color_to_value = color_to_value.clone();
    //
    // tracing::info!("file_path: {file_path}");
    // let path = std::path::Path::new(file_path);
    // let sheets = reader::xlsx::read(path).unwrap();
    // let items_sheet = sheets
    //     .get_sheet(&0)
    //     .map_err(|_| ERPError::ExcelError("商品sheet未找到".to_string()))?;
    //
    // let (cols, rows) = items_sheet.get_highest_column_and_row();
    //
    // let mut items = vec![];
    //
    // let mut pre: Option<ItemExcelDto> = None;
    //
    // // get colors first
    // let mut new_colors_to_empty_value = HashMap::new();
    // for i in 7..rows + 1 {
    //     let cell = items_sheet.get_cell((8, i));
    //     if cell.is_none() {
    //         continue;
    //     }
    //     let cell_value = cell.unwrap().get_raw_value().to_string();
    //     if cell_value.is_empty() {
    //         continue;
    //     }
    //     if !new_color_to_value.contains_key(&cell_value) {
    //         new_colors_to_empty_value.insert(cell_value, "");
    //     }
    // }
    // if !new_colors_to_empty_value.is_empty() {
    //     let new_colors = new_colors_to_empty_value
    //         .into_iter()
    //         .map(|item| item.0)
    //         .collect::<Vec<_>>();
    //
    //     if !new_colors.is_empty() {
    //         let c_to_v = state
    //             .settings_service
    //             .add_multiple_color_to_value(new_colors)
    //             .await?;
    //
    //         c_to_v.into_iter().for_each(|(c, v)| {
    //             new_color_to_value.insert(c, v);
    //         });
    //     }
    // }
    //
    // // 从第2行开始
    // for i in 7..rows + 1 {
    //     print!("row: {}", i);
    //
    //     let mut cur = ItemExcelDto::default();
    //     if let Some(previous) = pre {
    //         cur.images = previous.images;
    //         cur.number = previous.number;
    //         cur.size = previous.size;
    //         cur.name = previous.name;
    //     }
    //
    //     let mut images: Vec<&Image> = vec![];
    //     for j in 1..cols + 1 {
    //         // 图片在第3列
    //         if j == 3 {
    //             images = items_sheet.get_images((j, i));
    //             print!("images count: {}", images.len());
    //         }
    //
    //         let cell = items_sheet.get_cell((j, i));
    //         if cell.is_none() {
    //             if NONE_NULLABLE_JS.contains(&(j as i32)) {
    //                 if j == 2 && !cur.number.is_empty() || j == 5 && !cur.name.is_empty() {
    //                     continue;
    //                 }
    //
    //                 return Err(ERPError::ExcelError(format!(
    //                     "第{}行的 {} 为空/0",
    //                     i,
    //                     J_TO_NAME.get(&(j as i32)).unwrap_or(&"")
    //                 )));
    //             } else {
    //                 continue;
    //             }
    //         }
    //
    //         let cell_value = cell.unwrap().get_raw_value().to_string();
    //         if cell_value.is_empty() {
    //             if NONE_NULLABLE_JS.contains(&(j as i32)) {
    //                 if j == 2 && !cur.number.is_empty() || j == 5 && !cur.name.is_empty() {
    //                     continue;
    //                 }
    //
    //                 return Err(ERPError::ExcelError(format!(
    //                     "第{}行的 {} 为空",
    //                     i,
    //                     J_TO_NAME.get(&(j as i32)).unwrap_or(&"")
    //                 )));
    //             }
    //             continue;
    //         }
    //
    //         match j {
    //             2 => cur.number = cell_value.trim().to_string(),
    //             4 => cur.size = cell_value.trim().to_string(),
    //             5 => cur.name = cell_value.trim().to_string(),
    //             6 => cur.cates1 = cell_value.trim().to_string(),
    //             7 => cur.cates2 = cell_value.trim().to_string(),
    //             8 => cur.color = cell_value.trim().to_string().to_ascii_uppercase(),
    //             9 => cur.barcode = cell_value.trim().to_string(),
    //             10 => cur.count = (cell_value.parse::<f32>().unwrap_or(0.0) * 10.0) as i32,
    //             11 => cur.unit = cell_value.trim().to_string(),
    //             12 => cur.cost = (cell_value.parse::<f32>().unwrap_or(0.0) * 100.0) as i32,
    //             13 => cur.price = (cell_value.parse::<f32>().unwrap_or(0.0) * 100.0) as i32,
    //             15 => cur.notes = cell_value.trim().to_string(),
    //             _ => {}
    //         }
    //     }
    //
    //     if cur.color.is_empty() && cur.number.is_empty() && cur.cost == 0 && cur.price == 0 {
    //         break;
    //     }
    //
    //     if cur.barcode.is_empty() {
    //         let color_value = new_color_to_value.get(&cur.color).unwrap_or(&0);
    //         cur.barcode = calculate_barcode(&cur.number, *color_value, cur.price / 5);
    //     }
    //
    //     if cur.images.is_empty() && images.is_empty() {
    //         return Err(ERPError::ExcelError(format!("第{}行的 图片 为空", i)));
    //     }
    //     if !images.is_empty() {
    //         cur.images = match images.is_empty() {
    //             true => vec![],
    //             _ => {
    //                 let mut tmp = vec![];
    //                 for (index, real_goods_image) in images.into_iter().enumerate() {
    //                     let sku_image_name = format!("{}-{}.png", cur.barcode, index);
    //                     let goods_image_path =
    //                         format!("{}/sku/{}", STORAGE_FILE_PATH, sku_image_name);
    //                     real_goods_image.download_image(&goods_image_path);
    //                     tmp.push(format!("{}/sku/{}", STORAGE_URL_PREFIX, sku_image_name));
    //                 }
    //
    //                 tmp
    //             }
    //         };
    //     }
    //
    //     tracing::info!("rows#{:?}: {:?}", i, cur);
    //     pre = Some(cur.clone());
    //     items.push(cur);
    // }
    //
    // print!("rows: {rows:}: cols: {cols:}");
    //
    // Ok(items)
}
