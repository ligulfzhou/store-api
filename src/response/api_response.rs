use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

/// list response
#[derive(Debug, Serialize)]
pub struct ListResponse<T> {
    pub list: Vec<T>,
    pub total: i32,
}

#[derive(Debug, Serialize)]
pub struct APIListResponse<T: Serialize> {
    pub data: ListResponse<T>,
    pub code: i32,
    pub msg: String,
}

/// data response
#[derive(Debug, Serialize)]
pub struct APIDataResponse<T: Serialize> {
    pub data: T,
    pub code: i32,
    pub msg: String,
}

impl<T> APIDataResponse<T>
where
    T: Serialize,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            code: 0,
            msg: "".to_string(),
        }
    }
}

/// empty response
#[derive(Debug, Serialize)]
pub struct APIEmptyResponse {
    pub code: i32,
    pub msg: String,
}

impl APIEmptyResponse {
    pub fn new() -> Self {
        APIEmptyResponse {
            code: 0,
            msg: "".to_string(),
        }
    }
}

impl IntoResponse for APIEmptyResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl<T> APIListResponse<T>
where
    T: Serialize,
{
    pub fn new(list: Vec<T>, total: i32) -> Self {
        let list_response = ListResponse { list, total };
        APIListResponse {
            data: list_response,
            code: 0,
            msg: "".to_string(),
        }
    }
}

impl<T> IntoResponse for APIDataResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl<T> IntoResponse for APIListResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
