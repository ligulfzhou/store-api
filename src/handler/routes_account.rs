use crate::dto::dto_account::AccountDto;
use crate::response::api_response::APIDataResponse;
use crate::state::account_state::AccountState;
use crate::ERPResult;
use axum::extract::State;
use axum::routing::get;
use axum::{Extension, Router};

pub fn routes() -> Router<AccountState> {
    Router::new().route("/api/account/info", get(account_info))
}

pub async fn account_info(
    Extension(account): Extension<AccountDto>,
    State(_state): State<AccountState>,
) -> ERPResult<APIDataResponse<AccountDto>> {
    Ok(APIDataResponse::new(account))
}
