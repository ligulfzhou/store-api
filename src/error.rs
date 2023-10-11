use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::error::Error as SqlxError;
use thiserror::Error;

pub type ERPResult<T> = Result<T, ERPError>;

#[derive(Debug, Error)]
pub enum ERPError {
    /// 登陆相关
    #[error("登陆失败")]
    LoginFail,

    #[error("密码错误")]
    LoginFailForPasswordIsWrong,

    #[error("未登陆")]
    NotAuthorized,

    #[error("账号不存在")]
    AccountNotFound,

    #[error("无权限: {}", .0)]
    NoPermission(String),

    #[error("sqlx数据库错误: {:?}", .0)]
    DBError(#[from] SqlxError),

    #[error("数据已存在: {:?}", .0)]
    AlreadyExists(String),

    #[error("数据未找到: {:?}", .0)]
    NotFound(String),

    #[error("参数缺失: {:?}", .0)]
    ParamNeeded(String),

    #[error("参数错误: {:?}", .0)]
    ParamError(String),

    #[error("Excel数据有误: {:?}", .0)]
    ExcelError(String),

    #[error("json参数错误: {:?}", .0)]
    JsonExtractorRejection(#[from] JsonRejection),

    #[error("query参数错误: {:?}", .0)]
    QueryExtractorRejection(#[from] QueryRejection),

    #[error("{}", .0)]
    SaveFileFailed(String),

    #[error("参数错误: {:?}", .0)]
    ConvertFailed(String),

    #[error("{}", .0)]
    Failed(String),

    #[error("数据冲突: {}", .0)]
    Collision(String),
}

impl IntoResponse for ERPError {
    fn into_response(self) -> Response {
        print!("->> {:<12} - {self:?}", "INTO_RES");

        let msg = self.to_string();

        let code = match self {
            ERPError::NotAuthorized => 401,
            _ => 1,
        };

        (
            StatusCode::OK,
            serde_json::json!({
                "code": code, // failed code is always 1
                "msg": msg
            })
            .to_string(),
        )
            .into_response()
    }
}
