use crate::config::database::DatabaseTrait;
use crate::dto::dto_account::AccountDto;
use crate::dto::dto_excel::{EmbryoExcelDto, ItemExcelDto};
use crate::excel::parse_embryo::parse_embryos;
use crate::excel::parse_items::parse_items;
use crate::model::cates::CateModel;
use crate::model::embryo::{EmbryoInOutBucketModal, EmbryoInOutModel};
use crate::model::items::{ItemInOutBucketModal, ItemsInOutModel, ItemsModel};
use crate::response::api_response::APIEmptyResponse;
use crate::service::cates_service::CateServiceTrait;
use crate::service::embryo_service::EmbryoServiceTrait;
use crate::service::item_service::ItemServiceTrait;
use crate::service::settings_service::SettingsServiceTrait;
use crate::state::excel_state::ExcelState;
use crate::{ERPError, ERPResult};
use axum::extract::{Multipart, State};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Extension, Router};
use chrono::{Datelike, Timelike, Utc};
use rand::Rng;
use std::collections::HashMap;
use std::fs;

pub fn routes() -> Router<ExcelState> {
    Router::new()
        .route("/page/upload", get(page_upload_file))
        .route("/api/upload/excel", post(import_excel))
}

async fn page_upload_file(Extension(_account): Extension<AccountDto>) -> impl IntoResponse {
    Html(
        r#"
<!DOCTYPE html>
<html>
<body>

<form action="/api/upload/excel" method="post" enctype="multipart/form-data">
    Select image to upload:
    <input type="file" name="file" id="fileToUpload">
    <input type="submit" value="Upload Image" name="submit">
</form>

</body>
</html>
    "#,
    )
}

async fn import_excel(
    State(state): State<ExcelState>,
    Extension(account): Extension<AccountDto>,
    mut multipart: Multipart,
) -> ERPResult<APIEmptyResponse> {
    let mut file_path: String = "".to_string();
    let mut tp: i32 = 0;

    // 获取 二进制文件，保存到本地/tmp，并且目录是当前的时间
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name == "file" {
            let data = field.bytes().await.unwrap();
            let now = Utc::now();
            let mut rng = rand::thread_rng();
            let file_name = format!(
                "{}{:02}{:02}{:02}{:02}{:02}{:04}.xlsx",
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second(),
                rng.gen_range(0..9999)
            );
            tracing::info!("Length of `{}` is {} bytes", name, data.len());
            let file_path_full = format!("/tmp/{}", file_name);
            fs::write(&file_path_full, data).map_err(|_| {
                ERPError::SaveFileFailed(format!("create {} failed", file_path_full))
            })?;
            file_path = file_path_full;
        } else if name == "tp" {
            let data = String::from_utf8(field.bytes().await.unwrap().to_vec()).unwrap();
            tp = data.parse::<i32>().unwrap_or(0);
        }
    }

    // 检查文件是否保存成功了
    if file_path.is_empty() {
        return Err(ERPError::Failed("save excel file failed".to_string()));
    }

    let color_to_value = state
        .settings_service
        .get_all_color_to_values()
        .await?
        .into_iter()
        .map(|cv| (cv.color, cv.value))
        .collect::<HashMap<_, _>>();

    match tp {
        0 => process_item_excel(&state, &file_path, color_to_value, &account).await?,
        _ => process_embryo_excel(&state, &file_path, color_to_value, &account).await?,
    }

    Ok(APIEmptyResponse::new())
}

/// for embryo
async fn process_embryo_excel(
    state: &ExcelState,
    file_path: &str,
    _color_to_value: HashMap<String, i32>,
    account: &AccountDto,
) -> ERPResult<()> {
    tracing::info!("import excel for embryo....");
    let items = parse_embryos(&file_path)?;

    // 检查数据的正确性
    check_embryo_date_valid(&items)?;

    let mut number_to_id = HashMap::new();
    let number_to_count = items
        .iter()
        .map(|item| (item.number.clone(), item.count))
        .collect::<HashMap<_, _>>();
    let number_to_cost = items
        .iter()
        .map(|item| (item.number.clone(), item.cost))
        .collect::<HashMap<_, _>>();

    let numbers = items
        .iter()
        .map(|item| item.number.clone())
        .collect::<Vec<_>>();

    let existing_numbers = match numbers.len() {
        0 => vec![],
        _ => {
            let number_and_id = sqlx::query!(
                "select number, id from embryos where number = any($1)",
                &numbers
            )
            .fetch_all(state.db.get_pool())
            .await?;

            number_and_id.iter().for_each(|item| {
                number_to_id.insert(item.number.clone(), item.id);
            });

            number_and_id
                .into_iter()
                .map(|item| item.number)
                .collect::<Vec<_>>()
        }
    };

    let to_add_items = items
        .into_iter()
        .filter(|item| !existing_numbers.contains(&item.number))
        .collect::<Vec<_>>();

    if !to_add_items.is_empty() {
        state
            .embryo_service
            .insert_multiple_items(&to_add_items)
            .await?
            .iter()
            .for_each(|item| {
                number_to_id.insert(item.number.clone(), item.id);
            });
    }

    let bucket_id = state
        .embryo_service
        .add_inout_bucket(EmbryoInOutBucketModal {
            id: 0,
            account_id: account.id,
            in_true_out_false: true,
            via: "excel".to_string(),
            create_time: Default::default(),
        })
        .await?
        .id;

    let ins = number_to_count
        .into_iter()
        .map(|(number, count)| {
            let id = number_to_id.get(&number).unwrap_or(&0);
            let cost = number_to_cost.get(&number).unwrap_or(&0);
            let current_total = *cost * count;
            EmbryoInOutModel {
                id: 0,
                bucket_id,
                embryo_id: *id,
                count,
                current_cost: *cost,
                current_total,
            }
        })
        .collect::<Vec<_>>();

    if !ins.is_empty() {
        state
            .embryo_service
            .insert_multiple_items_inouts(&ins)
            .await?;
    }

    Ok(())
}

fn check_embryo_date_valid(items: &[EmbryoExcelDto]) -> ERPResult<()> {
    // 检查一下 excel里是否有 编号（number）重复

    Ok(())
}

/// for items

async fn process_item_excel(
    state: &ExcelState,
    file_path: &str,
    color_to_value: HashMap<String, i32>,
    account: &AccountDto,
) -> ERPResult<()> {
    tracing::info!("import excel....");
    let items = parse_items(state, &file_path, color_to_value).await?;
    if items.len() == 0 {
        return Ok(());
    }

    // 检查数据的正确性
    check_if_excel_data_valid(&state, &items).await?;

    // 对未出现过的 类别，入库(并返回所有的类别）
    let cate_data = handle_cates(&state, &items).await?;

    let mut barcode_to_id: HashMap<String, i32> = HashMap::new();
    let barcode_to_count = items
        .iter()
        .map(|item| (item.barcode.clone(), item.count))
        .collect::<HashMap<String, i32>>();
    let barcode_to_cost = items
        .iter()
        .map(|item| (item.barcode.clone(), item.cost))
        .collect::<HashMap<String, i32>>();

    // 用barcode去重
    let barcodes = items
        .iter()
        .filter_map(|item| {
            if item.barcode.is_empty() {
                None
            } else {
                Some(item.barcode.clone())
            }
        })
        .collect::<Vec<String>>();

    let existing_barcodes = match barcodes.len() {
        0 => vec![],
        _ => {
            let existing_items = sqlx::query!(
                "select barcode, id from items where barcode = any($1)",
                &barcodes
            )
            .fetch_all(state.db.get_pool())
            .await?;

            existing_items.iter().for_each(|item| {
                barcode_to_id.insert(item.barcode.clone(), item.id);
            });

            existing_items.into_iter().map(|r| r.barcode).collect()
        }
    };
    tracing::info!("existing_barcodes: {:?}", existing_barcodes);

    let to_add_items = items
        .into_iter()
        .filter(|item| !existing_barcodes.contains(&item.barcode))
        .collect::<Vec<ItemExcelDto>>();

    let empty_cate2_to_cate2_id: HashMap<String, i32> = HashMap::new();
    if !to_add_items.is_empty() {
        let mut item_models = vec![];
        for item in to_add_items {
            let cate1_id = *cate_data
                .existing_cate1_to_id
                .get(&item.cates1)
                .unwrap_or(&0);

            if cate1_id == 0 {
                return Err(ERPError::Failed(format!(
                    "大类 {} 未成功收录",
                    &item.cates1
                )));
            }

            let cate2_id = *cate_data
                .existing_cate1_id_to_cate2_to_cate2_id
                .get(&cate1_id)
                .unwrap_or(&empty_cate2_to_cate2_id)
                .get(&item.cates2)
                .unwrap_or(&0);

            if cate2_id == 0 {
                return Err(ERPError::Failed(format!(
                    "小类 {} 未成功收录",
                    &item.cates2
                )));
            }

            let item_model = ItemsModel {
                id: 0,
                color: item.color,
                cate1_id,
                cate2_id,
                name: item.name,
                size: item.size,
                unit: item.unit,
                price: item.price,
                barcode: item.barcode,
                notes: "".to_string(),
                images: item.images,
                create_time: Default::default(),
                cost: item.cost,
                number: item.number,
            };

            item_models.push(item_model)
        }

        tracing::info!("insert {:?} items", item_models.len());

        let mut st = 0;
        let l = 1000;
        while st < item_models.len() {
            if st + 1000 > item_models.len() {
                tracing::info!("{:?} \n", &item_models[st..].len());
                state
                    .item_service
                    .insert_multiple_items(&item_models[st..])
                    .await?
                    .into_iter()
                    .for_each(|item| {
                        barcode_to_id.insert(item.barcode.clone(), item.id);
                    });
            } else {
                tracing::info!("{:?} \n", &item_models[st..(st + 1000)].len());
                state
                    .item_service
                    .insert_multiple_items(&item_models[st..(st + 1000)])
                    .await?
                    .into_iter()
                    .for_each(|item| {
                        barcode_to_id.insert(item.barcode.clone(), item.id);
                    });
            }
            st += l;
        }
    }

    // 先添加库存bucket
    let bucket = state
        .item_service
        .add_inout_bucket(ItemInOutBucketModal {
            id: 0,
            account_id: account.id,
            in_true_out_false: true,
            via: "excel".to_string(),
            order_id: 0,
            create_time: Default::default(),
        })
        .await?;
    let bucket_id = bucket.id;

    // 添加库存
    let ins = barcode_to_count
        .into_iter()
        .map(|(barcode, count)| {
            let item_id = barcode_to_id.get(&barcode).unwrap_or(&0);
            let current_cost = barcode_to_cost.get(&barcode).unwrap_or(&0);
            let current_total = *current_cost * count;
            ItemsInOutModel {
                id: 0,
                bucket_id,
                item_id: *item_id,
                count,
                current_cost: *current_cost,
                current_total,
            }
        })
        .collect::<Vec<_>>();

    if !ins.is_empty() {
        state
            .item_service
            .insert_multiple_items_inouts(&ins, bucket_id)
            .await?;
    }

    Ok(())
}

async fn check_if_excel_data_valid(
    _state: &ExcelState,
    items: &[ItemExcelDto<'_>],
) -> ERPResult<()> {
    // 不能为空的字段，图片（可多张），名称，颜色，大类，单位，售价，成本，编号
    // no need

    // 编号必须是6位
    // 算了吧

    // 颜色必须预先录入
    // 已处理

    // barcode验证重复
    let mut barcode_to_count = HashMap::new();
    for item in items {
        let count = barcode_to_count.entry(&item.barcode).or_insert(0);
        *count += 1;
    }

    let dup_barcodes = barcode_to_count
        .iter()
        .filter_map(|(code, count)| {
            if count <= &1 {
                None
            } else {
                Some(code.as_str())
            }
        })
        .collect::<Vec<_>>();

    if !dup_barcodes.is_empty() {
        return Err(ERPError::ExcelError(format!(
            "重复的条形码有: {}",
            dup_barcodes.join(",")
        )));
    }

    Ok(())
}

struct CateData {
    existing_cate1_to_id: HashMap<String, i32>,
    existing_cate1_id_to_cate2_to_cate2_id: HashMap<i32, HashMap<String, i32>>,
}
async fn handle_cates(state: &ExcelState, items: &[ItemExcelDto<'_>]) -> ERPResult<CateData> {
    // 不需要处理，只需要把cates记录到数据库里
    // todo!()

    // 先处理类别
    let cates = sqlx::query_as!(CateModel, "select * from cates")
        .fetch_all(state.db.get_pool())
        .await
        .map_err(ERPError::DBError)?;

    let mut existing_cate1_to_id = cates
        .iter()
        .filter_map(|cate| match cate.cate_type {
            0 => Some((cate.name.clone(), cate.id)),
            _ => None,
        })
        .collect::<HashMap<String, i32>>();

    // let mut existing_cate1_id_to_cate2_to_cate2_id: HashMap<i32, HashMap<String, i32>> =
    let mut existing_cate1_id_to_cate2_to_cate2_id = HashMap::new();
    for cate in cates.iter() {
        if cate.cate_type == 0 {
            continue;
        }

        existing_cate1_id_to_cate2_to_cate2_id
            .entry(cate.parent_id)
            .or_insert(HashMap::new())
            .insert(cate.name.clone(), cate.id);
    }
    tracing::info!("existing existing_cate1_to_id: {:?}", existing_cate1_to_id);
    tracing::info!(
        "existing existing_cate1_id_to_cate2_to_cate2_id: {:?}",
        existing_cate1_id_to_cate2_to_cate2_id
    );

    let mut cate1_to_cate2s: HashMap<String, Vec<String>> = HashMap::new();
    for item in items.iter() {
        // todo: 必须得有cate1，然后才有cate2
        if item.cates1.is_empty() && !item.cates2.is_empty() {
            return Err(ERPError::ExcelError(
                "有数据，有小类，无大类，应该是有误".to_string(),
            ));
        }
        if item.cates1.is_empty() {
            continue;
        }
        let cate2s = cate1_to_cate2s.entry(item.cates1.clone()).or_insert(vec![]);

        if !item.cates2.is_empty() && !cate2s.contains(&item.cates2) {
            cate2s.push(item.cates2.clone());
        }
    }

    // 添加cate1
    let to_add_cate1s = cate1_to_cate2s
        .iter()
        .filter_map(|(k, _)| {
            if existing_cate1_to_id.contains_key(k) {
                None
            } else {
                Some(k.as_str())
            }
        })
        .collect::<Vec<&str>>();
    if !to_add_cate1s.is_empty() {
        let new_cate1_to_id = state
            .cate_service
            .insert_multiple_cate1(&to_add_cate1s)
            .await?;
        for (k, v) in new_cate1_to_id {
            existing_cate1_to_id.insert(k, v);
        }
    }

    let empty_hash = HashMap::new();
    let mut to_add_cate2s = vec![];
    for (cate1, cate2s) in cate1_to_cate2s.iter() {
        let cate1_id = existing_cate1_to_id.get(cate1).ok_or(ERPError::ExcelError(
            "有点问题，如果在次出现，找周".to_string(),
        ))?;

        for cate2 in cate2s {
            if existing_cate1_id_to_cate2_to_cate2_id
                .get(cate1_id)
                .unwrap_or(&empty_hash)
                .contains_key(cate2)
            {
                continue;
            }

            to_add_cate2s.push(CateModel {
                id: 0,
                index: 0,
                name: cate2.to_string(),
                cate_type: 1,
                parent_id: *cate1_id,
                create_time: Default::default(),
            })
        }
    }
    if !to_add_cate2s.is_empty() {
        let new_cates = state
            .cate_service
            .insert_multiple_cate2(&to_add_cate2s)
            .await?;
        for new_cate in new_cates {
            existing_cate1_id_to_cate2_to_cate2_id
                .entry(new_cate.parent_id)
                .or_insert(HashMap::new())
                .insert(new_cate.name, new_cate.id);
        }
    }

    tracing::info!("cate1_to_cate2s: {:?}", cate1_to_cate2s);
    tracing::info!("existing_cate1_to_id: {:?}", existing_cate1_to_id);
    tracing::info!(
        "existing_cate1_id_to_cate2_to_cate2_id: {:?}",
        existing_cate1_id_to_cate2_to_cate2_id
    );

    Ok(CateData {
        existing_cate1_to_id,
        existing_cate1_id_to_cate2_to_cate2_id,
    })
}
