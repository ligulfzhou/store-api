use crate::dto::dto_excel::ItemExcelDto;
use crate::excel::parse_items::parse_items;
use crate::model::cates::CatesModel;
use crate::model::items::ItemsModel;
use crate::response::api_response::APIEmptyResponse;
use crate::{AppState, ERPError, ERPResult};
use axum::extract::{Multipart, State};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::Router;
use chrono::format::Item;
use chrono::{Datelike, Timelike, Utc};
use itertools::Itertools;
use rand::Rng;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/page/upload", get(page_upload_file))
        .route("/api/upload/excel", post(import_excel))
        .with_state(state)
}

async fn page_upload_file() -> impl IntoResponse {
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
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> ERPResult<APIEmptyResponse> {
    let mut file_path: String = "".to_string();

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
        }
    }

    // 检查文件是否保存成功了
    if file_path.is_empty() {
        return Err(ERPError::Failed("save excel file failed".to_string()));
    }

    tracing::info!("import excel....");
    let items = parse_items(&file_path)?;

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
        _ => sqlx::query!(
            "select barcode from items where barcode = any($1)",
            &barcodes
        )
        .fetch_all(&state.db)
        .await
        .map_err(ERPError::DBError)?
        .into_iter()
        .map(|r| r.barcode)
        .collect(),
    };
    tracing::info!("existing_barcodes: {:?}", existing_barcodes);

    let goods_nos = items
        .iter()
        .filter_map(|item| {
            if item.goods_no.is_empty() {
                None
            } else {
                Some(item.goods_no.clone())
            }
        })
        .collect::<Vec<String>>();

    let existing_goods_nos_with_color = match goods_nos.len() {
        0 => vec![],
        _ => sqlx::query!(
            "select goods_no, color from items where goods_no = any($1)",
            &goods_nos
        )
        .fetch_all(&state.db)
        .await
        .map_err(ERPError::DBError)?
        .into_iter()
        .map(|r| format!("{}-{}", r.goods_no, r.color))
        .collect::<Vec<String>>(),
    };
    tracing::info!(
        "existing_goods_nos_with_color: {:?}",
        existing_goods_nos_with_color
    );

    let to_add_items = items
        .into_iter()
        .filter(|item| {
            !existing_barcodes.contains(&item.barcode)
                && !existing_goods_nos_with_color
                    .contains(&format!("{}-{}", item.goods_no, item.color))
        })
        .collect::<Vec<ItemExcelDto>>();

    if !to_add_items.is_empty() {
        let cate_data = handle_cates(&state.db, &to_add_items).await?;

        let item_models = to_add_items
            .into_iter()
            .map(|item| {
                let cates1_id = *cate_data
                    .existing_cate1_to_id
                    .get(&item.cates1)
                    .unwrap_or(&0);
                let cates2_id = match cates1_id {
                    0 => 0,
                    _ => *cate_data
                        .existing_cate1_id_to_cate2_to_cate2_id
                        .get(&cates1_id)
                        .unwrap_or(&HashMap::new())
                        .get(&item.cates2)
                        .unwrap_or(&0),
                };
                ItemsModel {
                    id: 0,
                    brand: item.brand,
                    cates1_id,
                    cates2_id,
                    goods_no: item.goods_no,
                    color: item.color,
                    name: item.name,
                    size: item.size,
                    unit: item.unit,
                    barcode: item.barcode,
                    sell_price: item.sell_price,
                    buy_price: item.buy_price,
                }
            })
            .collect::<Vec<ItemsModel>>();

        tracing::info!("insert {:?} items", item_models.len());

        let mut st = 0;
        let l = 1000;
        while st < item_models.len() {
            if st + 1000 > item_models.len() {
                tracing::info!("{:?} \n", &item_models[st..].len());
                ItemsModel::insert_multiple_items(&state.db, &item_models[st..]).await?;
            } else {
                tracing::info!("{:?} \n", &item_models[st..(st + 1000)].len());
                ItemsModel::insert_multiple_items(&state.db, &item_models[st..(st + 1000)]).await?;
            }
            st += l;

            sleep(Duration::new(5, 0));
        }

        // ItemsModel::insert_multiple_items(&state.db, &item_models).await?;
    }

    Ok(APIEmptyResponse::new())
}

struct CateData {
    existing_cate1_to_id: HashMap<String, i32>,
    existing_cate1_id_to_cate2_to_cate2_id: HashMap<i32, HashMap<String, i32>>,
}

async fn handle_cates(db: &Pool<Postgres>, items: &[ItemExcelDto]) -> ERPResult<CateData> {
    // 先处理类别
    let cates = sqlx::query_as!(CatesModel, "select * from cates")
        .fetch_all(db)
        .await
        .map_err(ERPError::DBError)?;

    let mut existing_cate1_to_id = cates
        .iter()
        .filter_map(|cate| match cate.cate_type {
            0 => Some((cate.name.clone(), cate.id)),
            _ => None,
        })
        .collect::<HashMap<String, i32>>();

    let mut existing_cate1_id_to_cate2_to_cate2_id: HashMap<i32, HashMap<String, i32>> =
        HashMap::new();
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
                Some(k.clone())
            }
        })
        .collect::<Vec<String>>();
    if !to_add_cate1s.is_empty() {
        let new_cate1_to_id = CatesModel::insert_multiple_cate1(db, &to_add_cate1s).await?;
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

            to_add_cate2s.push(CatesModel {
                id: 0,
                name: cate2.to_string(),
                cate_type: 1,
                parent_id: *cate1_id,
            })
        }
    }
    if !to_add_cate2s.is_empty() {
        let new_cates = CatesModel::insert_multiple_cate2(db, to_add_cate2s).await?;
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
