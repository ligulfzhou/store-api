use crate::dto::dto_account::AccountDto;
use crate::model::account::{AccountModel, DepartmentModel};
use crate::response::api_response::{APIDataResponse, APIEmptyResponse};
use crate::{AppState, ERPError, ERPResult};
use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;
use axum::{routing::post, Json, Router};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use std::sync::Arc;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .route("/api/logout", post(api_logout))
        .with_state(state)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    pub account: String,
    pub password: String,
}

async fn api_login(
    State(state): State<Arc<AppState>>,
    WithRejection(Json(payload), _): WithRejection<Json<LoginPayload>, ERPError>,
) -> Result<impl IntoResponse, ERPError> {
    tracing::info!("->> {:<12}, api_login", "handler");
    let account = sqlx::query_as!(
        AccountModel,
        "select * from accounts where account=$1",
        payload.account
    )
    .fetch_optional(&state.db)
    .await
    .map_err(ERPError::DBError)?;

    if account.is_none() {
        return Err(ERPError::NotFound("账号不存在".to_string()));
    }

    let account_unwrap = account.unwrap();
    // todo: hash password.
    if account_unwrap.password != payload.password {
        return Err(ERPError::LoginFailForPasswordIsWrong);
    }

    let account_id = account_unwrap.id;
    let department = sqlx::query_as!(
        DepartmentModel,
        "select * from departments where id=$1",
        account_unwrap.department_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(ERPError::DBError)?;

    let account_dto = AccountDto::from(account_unwrap, department);

    let cookie = Cookie::build("account_id", account_id.to_string())
        .path("/")
        .max_age(time::Duration::days(365 * 10))
        .same_site(SameSite::None)
        .domain("")
        .http_only(true)
        .secure(true)
        .finish();

    let mut response = APIDataResponse::new(account_dto).into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

async fn api_logout() -> ERPResult<impl IntoResponse> {
    let cookie = Cookie::build("account_id", "")
        .path("/")
        .max_age(time::Duration::hours(-1))
        .domain("")
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true)
        .finish();

    let mut response = APIEmptyResponse::new().into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::handler::routes_login::LoginPayload;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let param = LoginPayload {
            account: "test".to_string(),
            password: "test".to_string(),
        };
        let client = httpc_test::new_client("http://localhost:9100")?;
        client
            .do_post("/api/login", serde_json::json!(param))
            .await?
            .print()
            .await?;

        client.do_get("/api/account/info").await?.print().await?;
        Ok(())
    }
}
