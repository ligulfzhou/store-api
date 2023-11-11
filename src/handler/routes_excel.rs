use crate::config::database::DatabaseTrait;
use crate::dto::dto_excel::ItemExcelDto;
use crate::excel::parse_items::parse_items;
use crate::model::items::ItemsModel;
use crate::response::api_response::APIEmptyResponse;
use crate::service::item_service::ItemServiceTrait;
use crate::state::item_state::ItemState;
use crate::{ERPError, ERPResult};
use axum::extract::{Multipart, State};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::Router;
use chrono::{Datelike, Timelike, Utc};
use rand::Rng;
use sqlx::{Pool, Postgres};
use std::fs;

pub fn routes() -> Router<ItemState> {
    Router::new()
        .route("/page/upload", get(page_upload_file))
        .route("/api/upload/excel", post(import_excel))
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
    State(state): State<ItemState>,
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
        .fetch_all(state.db.get_pool())
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
        .fetch_all(state.db.get_pool())
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
        let item_models = to_add_items
            .into_iter()
            .map(|item| ItemsModel {
                id: 0,
                color: item.color,
                // todo
                cate1_id: 0,
                name: item.name,
                size: item.size,
                unit: item.unit,
                // todo
                price: 0,
                barcode: item.barcode,
                notes: "".to_string(),
                images: vec![],
                create_time: Default::default(),
                cate2_id: 0,
                cost: 0,
                number: "".to_string(),
            })
            .collect::<Vec<ItemsModel>>();

        tracing::info!("insert {:?} items", item_models.len());

        let mut st = 0;
        let l = 1000;
        while st < item_models.len() {
            if st + 1000 > item_models.len() {
                tracing::info!("{:?} \n", &item_models[st..].len());
                state
                    .item_service
                    .insert_multiple_items(&item_models[st..])
                    .await?;
            } else {
                tracing::info!("{:?} \n", &item_models[st..(st + 1000)].len());
                state
                    .item_service
                    .insert_multiple_items(&item_models[st..(st + 1000)])
                    .await?;
            }
            st += l;
        }
    }

    Ok(APIEmptyResponse::new())
}

async fn handle_cates(db: &Pool<Postgres>, items: &[ItemExcelDto]) -> ERPResult<()> {
    // 不需要处理，只需要把cates记录到数据库里
    todo!()

    // // 先处理类别
    // let cates = sqlx::query_as!(CateModel, "select * from cates")
    //     .fetch_all(db)
    //     .await
    //     .map_err(ERPError::DBError)?;
    //
    // let mut existing_cate1_to_id = cates
    //     .iter()
    //     .filter_map(|cate| match cate.cate_type {
    //         0 => Some((cate.name.clone(), cate.id)),
    //         _ => None,
    //     })
    //     .collect::<HashMap<String, i32>>();
    //
    // let mut existing_cate1_id_to_cate2_to_cate2_id: HashMap<i32, HashMap<String, i32>> =
    //     HashMap::new();
    // for cate in cates.iter() {
    //     if cate.cate_type == 0 {
    //         continue;
    //     }
    //
    //     existing_cate1_id_to_cate2_to_cate2_id
    //         .entry(cate.parent_name)
    //         .or_insert(HashMap::new())
    //         .insert(cate.name.clone(), cate.id);
    // }
    // tracing::info!("existing existing_cate1_to_id: {:?}", existing_cate1_to_id);
    // tracing::info!(
    //     "existing existing_cate1_id_to_cate2_to_cate2_id: {:?}",
    //     existing_cate1_id_to_cate2_to_cate2_id
    // );
    //
    // let mut cate1_to_cate2s: HashMap<String, Vec<String>> = HashMap::new();
    // for item in items.iter() {
    //     // todo: 必须得有cate1，然后才有cate2
    //     if item.cates1.is_empty() && !item.cates2.is_empty() {
    //         return Err(ERPError::ExcelError(
    //             "有数据，有小类，无大类，应该是有误".to_string(),
    //         ));
    //     }
    //     if item.cates1.is_empty() {
    //         continue;
    //     }
    //     let cate2s = cate1_to_cate2s.entry(item.cates1.clone()).or_insert(vec![]);
    //
    //     if !item.cates2.is_empty() && !cate2s.contains(&item.cates2) {
    //         cate2s.push(item.cates2.clone());
    //     }
    // }
    //
    // // 添加cate1
    // let to_add_cate1s = cate1_to_cate2s
    //     .iter()
    //     .filter_map(|(k, _)| {
    //         if existing_cate1_to_id.contains_key(k) {
    //             None
    //         } else {
    //             Some(k.clone())
    //         }
    //     })
    //     .collect::<Vec<String>>();
    // if !to_add_cate1s.is_empty() {
    //     let new_cate1_to_id = CatesModel::insert_multiple_cate1(db, &to_add_cate1s).await?;
    //     for (k, v) in new_cate1_to_id {
    //         existing_cate1_to_id.insert(k, v);
    //     }
    // }
    //
    // let empty_hash = HashMap::new();
    // let mut to_add_cate2s = vec![];
    // for (cate1, cate2s) in cate1_to_cate2s.iter() {
    //     let cate1_id = existing_cate1_to_id.get(cate1).ok_or(ERPError::ExcelError(
    //         "有点问题，如果在次出现，找周".to_string(),
    //     ))?;
    //
    //     for cate2 in cate2s {
    //         if existing_cate1_id_to_cate2_to_cate2_id
    //             .get(cate1_id)
    //             .unwrap_or(&empty_hash)
    //             .contains_key(cate2)
    //         {
    //             continue;
    //         }
    //
    //         to_add_cate2s.push(CateModel {
    //             id: 0,
    //             name: cate2.to_string(),
    //             cate_type: 1,
    //             parent_name: *cate1_id,
    //         })
    //     }
    // }
    // if !to_add_cate2s.is_empty() {
    //     let new_cates = CateModel::insert_multiple_cate2(db, to_add_cate2s).await?;
    //     for new_cate in new_cates {
    //         existing_cate1_id_to_cate2_to_cate2_id
    //             .entry(new_cate.parent_name)
    //             .or_insert(HashMap::new())
    //             .insert(new_cate.name, new_cate.id);
    //     }
    // }
    //
    // tracing::info!("cate1_to_cate2s: {:?}", cate1_to_cate2s);
    // tracing::info!("existing_cate1_to_id: {:?}", existing_cate1_to_id);
    // tracing::info!(
    //     "existing_cate1_id_to_cate2_to_cate2_id: {:?}",
    //     existing_cate1_id_to_cate2_to_cate2_id
    // );
    //
    // Ok(CateData {
    //     existing_cate1_to_id,
    //     existing_cate1_id_to_cate2_to_cate2_id,
    // })
}
