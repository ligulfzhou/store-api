use crate::constants::DEFAULT_PAGE_SIZE;
use crate::dto::dto_account::AccountDto;
use crate::dto::dto_items::{
    DeleteParams, EditParams, InoutParams, InoutQueryParams, ItemInOutDto, ItemSearchParams,
    ItemsDto, QueryParams,
};
use crate::model::items::ItemsModel;
use crate::response::api_response::{APIEmptyResponse, APIListResponse};
use crate::service::item_service::ItemServiceTrait;
use crate::state::item_state::ItemState;
use crate::{ERPError, ERPResult};
use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::WithRejection;

pub fn routes() -> Router<ItemState> {
    Router::new()
        .route("/api/items", get(api_item_list))
        .route("/api/item/edit", post(api_item_edit))
        .route("/api/item/delete", post(api_item_delete))
        .route("/api/item/stock", get(api_item_stock))
        .route("/api/item/inout", post(api_item_inout))
        .route("/api/item/inout/list", get(api_inout_list))
        .route("/api/item/search", get(api_item_list))
}

async fn api_item_list(
    State(state): State<ItemState>,
    WithRejection(Query(params), _): WithRejection<Query<QueryParams>, ERPError>,
) -> ERPResult<APIListResponse<ItemsDto>> {
    let items = state.item_service.get_item_list(&params).await?;
    let count = state.item_service.get_item_count(&params).await?;
    Ok(APIListResponse::new(items, count))
}

async fn api_item_edit(
    State(state): State<ItemState>,
    WithRejection(Json(params), _): WithRejection<Json<EditParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    state.item_service.edit_item(&params).await?;
    Ok(APIEmptyResponse::new())
}

async fn api_item_delete(
    State(state): State<ItemState>,
    WithRejection(Json(params), _): WithRejection<Json<DeleteParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    state.item_service.delete_item(&params).await?;
    Ok(APIEmptyResponse::new())
}

async fn api_item_stock(
    State(state): State<ItemState>,
    WithRejection(Query(params), _): WithRejection<Query<QueryParams>, ERPError>,
) -> ERPResult<APIListResponse<ItemsDto>> {
    // let items = state.item_service.get_item_list(&params).await?;
    // let items_dto = state.item_service.to_items_dto(items).await?;
    // let count = state.item_service.get_item_count(&params).await?;
    // Ok(APIListResponse::new(items_dto, count))

    todo!()
}

async fn api_item_inout(
    State(state): State<ItemState>,
    Extension(account): Extension<AccountDto>,
    WithRejection(Json(params), _): WithRejection<Json<InoutParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    tracing::info!("api_item_inout : /api/item/inout");
    state
        .item_service
        .add_item_inout(&params, account.id)
        .await?;
    Ok(APIEmptyResponse::new())
}

async fn api_inout_list(
    State(state): State<ItemState>,
    Extension(account): Extension<AccountDto>,
    WithRejection(Query(params), _): WithRejection<Query<InoutQueryParams>, ERPError>,
) -> ERPResult<APIListResponse<ItemInOutDto>> {
    tracing::info!("api_item_list : /api/item/inout/list");

    let items = state
        .item_service
        .inout_list_of_item(
            params.item_id,
            &account.name,
            params.page.unwrap_or(1),
            params.page_size.unwrap_or(DEFAULT_PAGE_SIZE),
        )
        .await?;

    let count = state
        .item_service
        .inout_list_of_item_count(params.item_id)
        .await?;
    Ok(APIListResponse::new(items, count))
}

async fn api_item_search(
    State(state): State<ItemState>,
    WithRejection(Query(params), _): WithRejection<Query<ItemSearchParams>, ERPError>,
) -> ERPResult<APIListResponse<ItemsModel>> {
    tracing::info!("api_item_list : /api/item/search");

    let items = state.item_service.search_item(&params).await?;
    let len = items.len() as i32;
    Ok(APIListResponse::new(items, len))
}
