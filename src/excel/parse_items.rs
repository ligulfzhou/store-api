use crate::dto::dto_excel::ItemExcelDto;
use crate::{ERPError, ERPResult};
use umya_spreadsheet::*;

/*
品牌	产品大类	产品小类	货号	产品名称	规格	单位	条码	标准售价	进货价	供应商	库存数	修改时间	描述	备注
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
    for i in 2..rows {
        let mut cur = ItemExcelDto::default();

        for j in 1..cols + 1 {
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
            // tracing::info!("cell({:?}, {:?}): {:?}", i, j, cell_value);

            // todo: 解析code
            match j {
                1 => cur.brand = cell_value.trim().to_string(),
                2 => cur.cates1 = cell_value.trim().to_string(),
                3 => cur.cates2 = cell_value.trim().to_string(),
                // 4 => {
                //     let str_value = cell_value.trim().to_string();
                //     let code_with_color = str_value.split("-").collect::<Vec<&str>>();
                //     if code_with_color.len() < 2 {
                //         return Err(ERPError::ExcelError(format!("第{:?}行的货号数据有问题", i)));
                //     }
                //     cur.goods_no = code_with_color[0].to_string();
                //     cur.color = code_with_color[1].to_string();
                // }
                // 5 => cur.name = cell_value.trim().to_string(),
                4 => {}
                5 => {
                    let str_value = cell_value.trim().to_string();
                    let code_with_color = str_value.split("-").collect::<Vec<&str>>();
                    cur.goods_no = code_with_color[0].to_string();

                    if code_with_color.len() >= 2 {
                        cur.color = code_with_color[1].to_string();
                    }
                    cur.name = cell_value.trim().to_string();
                }

                6 => cur.size = cell_value.trim().to_string(),
                7 => cur.unit = cell_value.trim().to_string(),
                8 => cur.barcode = cell_value.trim().to_string(),
                9 => cur.sell_price = (cell_value.parse::<f32>().unwrap_or(0.0) * 100.0) as i32,
                10 => cur.buy_price = (cell_value.parse::<f32>().unwrap_or(0.0) * 100.0) as i32,
                _ => {}
            }
        }

        // tracing::info!("rows#{:?}: {:?}", i, cur);
        items.push(cur);
    }

    print!("rows: {rows:}: cols: {cols:}");

    Ok(items)
}
