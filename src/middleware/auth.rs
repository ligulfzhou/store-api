use crate::dto::dto_account::AccountDto;
use crate::model::account::{AccountModel, DepartmentModel};
use crate::{AppState, ERPError};
use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};
use axum_extra::extract::cookie::CookieJar;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn auth<B>(
    cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, ERPError> {
    let account_id = cookie_jar
        .get("account_id")
        .map(|cookie| cookie.value().to_string());
    if account_id.is_none() {
        return Err(ERPError::NotAuthorized);
    }

    let account_id = account_id.unwrap();

    let account = sqlx::query_as::<_, AccountModel>(&format!(
        "select * from accounts where id={}",
        account_id
    ))
    .fetch_optional(&state.db)
    .await
    .map_err(ERPError::DBError)?;

    if account.is_none() {
        return Err(ERPError::AccountNotFound);
    }

    let account = account.unwrap();
    let department = sqlx::query_as::<_, DepartmentModel>(&format!(
        "select * from departments where id={}",
        account.department_id
    ))
    .fetch_one(&state.db)
    .await
    .map_err(ERPError::DBError)?;

    let account_dto = AccountDto::from(account, department);

    req.extensions_mut().insert(account_dto);
    Ok(next.run(req).await)
}
