use crate::dto::dto_account::AccountDto;
use crate::dto::dto_orders::{CreateOrderParams, OrderDto};
use crate::response::api_response::{APIDataResponse, APIEmptyResponse};
use crate::service::order_service::OrderServiceTrait;
use crate::service::settings_service::SettingsServiceTrait;
use crate::state::order_state::OrderState;
use crate::{ERPError, ERPResult};
use axum::{
    extract::State,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::WithRejection;

pub fn routes() -> Router<OrderState> {
    Router::new()
        .route("/api/orders", get(api_create_order))
        .route("/api/orders/create", post(api_create_order))
}

async fn api_order_list(
    State(state): State<OrderState>,
    Extension(account): Extension<AccountDto>,
) -> ERPResult<APIDataResponse<OrderDto>> {
    todo!()
}

#[derive(Serialize)]
struct OrderId {
    id: i32,
}

async fn api_create_order(
    State(state): State<OrderState>,
    Extension(account): Extension<AccountDto>,
    WithRejection(Json(params), _): WithRejection<Json<CreateOrderParams>, ERPError>,
) -> ERPResult<APIDataResponse<OrderId>> {
    let order_id = state
        .order_service
        .create_order(account.id, &params)
        .await?;

    Ok(APIDataResponse::new(OrderId { id: order_id }))
}
