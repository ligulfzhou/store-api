use crate::constants::{STORAGE_FILE_PATH, STORAGE_URL_PREFIX};
use crate::response::api_response::APIDataResponse;
use crate::{ERPError, ERPResult};
use axum::extract::Multipart;
use axum::routing::post;
use axum::Router;
use chrono::{Datelike, Timelike, Utc};
use std::fs;

pub fn routes() -> Router {
    Router::new().route("/api/upload/image", post(upload_image))
}

#[derive(Debug, Serialize)]
struct ImageUrlResponse {
    url: String,
}

async fn upload_image(mut multipart: Multipart) -> ERPResult<APIDataResponse<ImageUrlResponse>> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        tracing::info!("field name: {}", name);
        if name == "file" {
            let data = field.bytes().await.unwrap();
            let now = Utc::now();
            let ymd = format!("{}{:02}{:02}", now.year(), now.month(), now.day());
            let ymdhms = format!(
                "{}{:02}{:02}{:02}",
                ymd,
                now.hour(),
                now.minute(),
                now.second()
            );
            // let file_suffix =
            let dir_path = format!("{}/images/{}", STORAGE_FILE_PATH, ymd);
            tracing::info!("dir_path: {}", dir_path);
            let file_name = format!("{}.png", ymdhms);
            tracing::info!("filename: {}", file_name);
            fs::create_dir_all(&dir_path)
                .map_err(|_| ERPError::SaveFileFailed(format!("create {} failed", dir_path)))?;
            tracing::info!("Length of `{}` is {} bytes", name, data.len());
            let file_path_full = format!("{}/{}", dir_path, file_name);
            fs::write(&file_path_full, data).map_err(|_| {
                ERPError::SaveFileFailed(format!("create {} failed", file_path_full))
            })?;

            let url = format!("{}/images/{}/{}", STORAGE_URL_PREFIX, ymd, file_name);
            return Ok(APIDataResponse::new(ImageUrlResponse { url }));
        }
    }

    Err(ERPError::SaveFileFailed("文件存储失败".to_string()))
}
