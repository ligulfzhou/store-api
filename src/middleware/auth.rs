use crate::config::database::DatabaseTrait;
use crate::dto::dto_account::AccountDto;
use crate::model::account::AccountModel;
use crate::state::account_state::AccountState;
use crate::ERPError;
use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};
use axum_extra::extract::cookie::CookieJar;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn auth<B>(
    cookie_jar: CookieJar,
    State(state): State<AccountState>,
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
    .fetch_optional(state.account_repo.db.get_pool())
    .await
    .map_err(ERPError::DBError)?;

    if account.is_none() {
        return Err(ERPError::AccountNotFound);
    }

    let account = account.unwrap();
    // let department = sqlx::query_as::<_, DepartmentModel>(&format!(
    //     "select * from departments where id={}",
    //     account.department_id
    // ))
    // .fetch_one(state.account_repo.db.get_pool())
    // .await
    // .map_err(ERPError::DBError)?;

    let account_dto = AccountDto::from(account);

    req.extensions_mut().insert(account_dto);
    Ok(next.run(req).await)
}
