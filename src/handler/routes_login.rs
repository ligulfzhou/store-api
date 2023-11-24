use crate::dto::dto_account::AccountDto;
use crate::response::api_response::{APIDataResponse, APIEmptyResponse};
use crate::state::account_state::AccountState;
use crate::{ERPError, ERPResult};
use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;
use axum::{routing::post, Json, Router};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::WithRejection;
use serde::Deserialize;

pub fn routes() -> Router<AccountState> {
    Router::new()
        .route("/api/login", post(api_login))
        .route("/api/logout", post(api_logout))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    pub account: String,
    pub password: String,
}

async fn api_login(
    State(state): State<AccountState>,
    WithRejection(Json(payload), _): WithRejection<Json<LoginPayload>, ERPError>,
) -> Result<impl IntoResponse, ERPError> {
    tracing::info!("->> {:<12}, api_login", "handler");

    let account = state
        .account_repo
        .find_user_by_account(&payload.account)
        .await
        .ok_or(ERPError::NotFound("账号不存在".to_string()))?;

    // todo: hash password.
    if account.password != payload.password {
        return Err(ERPError::LoginFailForPasswordIsWrong);
    }

    let account_id = account.id;
    let account_dto = AccountDto::from(account);

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

async fn api_logout(State(_state): State<AccountState>) -> ERPResult<impl IntoResponse> {
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
    use crate::ERPResult;

    #[tokio::test]
    async fn test() -> ERPResult<()> {
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
