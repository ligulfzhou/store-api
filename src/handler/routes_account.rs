use crate::dto::dto_account::AccountDto;
use crate::middleware::auth::auth;
use crate::response::api_response::APIDataResponse;
use crate::{AppState, ERPResult};
use axum::extract::State;
use axum::middleware;
use axum::routing::get;
use axum::{Extension, Router};
use std::sync::Arc;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/account/info", get(account_info))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state)
}

pub async fn account_info(
    Extension(account): Extension<AccountDto>,
    State(_state): State<Arc<AppState>>,
) -> ERPResult<APIDataResponse<AccountDto>> {
    Ok(APIDataResponse::new(account))
}
