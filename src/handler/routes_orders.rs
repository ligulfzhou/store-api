use crate::dto::dto_account::AccountDto;
use crate::dto::dto_orders::{
    CreateOrderParams, DeleteOrderParams, OrderDetailDto, OrderDetailQueryParams, OrderInListDto,
    QueryParams,
};
use crate::response::api_response::{APIDataResponse, APIEmptyResponse, APIListResponse};
use crate::service::order_service::OrderServiceTrait;
use crate::state::order_state::OrderState;
use crate::{ERPError, ERPResult};
use axum::extract::Query;
use axum::{
    extract::State,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::WithRejection;

pub fn routes() -> Router<OrderState> {
    Router::new()
        .route("/api/orders/list", get(api_order_list))
        .route("/api/imported/orders/list", get(api_imported_order_list))
        .route("/api/orders/create", post(api_create_order))
        .route("/api/order/detail", get(api_order_detail))
        .route("/api/order/delete", post(api_order_delete))
}

async fn api_order_detail(
    State(state): State<OrderState>,
    Extension(_): Extension<AccountDto>,
    WithRejection(Query(params), _): WithRejection<Query<OrderDetailQueryParams>, ERPError>,
) -> ERPResult<APIDataResponse<OrderDetailDto>> {
    let order_dto = state.order_service.get_order(params.order_id).await?;
    let order_items_dtos = state.order_service.get_order_items(params.order_id).await?;

    Ok(APIDataResponse::new(OrderDetailDto {
        order: order_dto,
        items: order_items_dtos,
    }))
}

async fn api_order_list(
    State(state): State<OrderState>,
    Extension(_account): Extension<AccountDto>,
    WithRejection(Query(params), _): WithRejection<Query<QueryParams>, ERPError>,
) -> ERPResult<APIListResponse<OrderInListDto>> {
    tracing::info!("api_order_list...");
    let orders = state.order_service.get_order_list(&params).await?;
    tracing::info!("orders.len: {}", orders.len());
    let count = state.order_service.get_count_order_list(&params).await?;
    tracing::info!("orders.count: {}", count);

    Ok(APIListResponse::new(orders, count))
}

async fn api_imported_order_list(
    State(state): State<OrderState>,
    Extension(_account): Extension<AccountDto>,
    WithRejection(Query(params), _): WithRejection<Query<QueryParams>, ERPError>,
) -> ERPResult<APIListResponse<OrderInListDto>> {
    tracing::info!("api_order_list...");
    let orders = state.order_service.get_imported_order_list(&params).await?;
    tracing::info!("orders.len: {}", orders.len());
    let count = state
        .order_service
        .get_count_imported_order_list(&params)
        .await?;
    tracing::info!("orders.count: {}", count);

    Ok(APIListResponse::new(orders, count))
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
    tracing::info!("api_create_order...");

    let order_id = state
        .order_service
        .create_order(account.id, &params)
        .await?;

    Ok(APIDataResponse::new(OrderId { id: order_id }))
}

async fn api_order_delete(
    State(state): State<OrderState>,
    Extension(_): Extension<AccountDto>,
    WithRejection(Json(params), _): WithRejection<Json<DeleteOrderParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    let order = state.order_service.get_order(params.id).await?;
    match order.tp {
        0 => {
            // 正常订单
            state.order_service.delete_order(params.id).await?;
        }
        _ => {
            // 导入订单
            state.order_service.delete_import_order(params.id).await?;
        }
    }

    Ok(APIEmptyResponse::new())
}
