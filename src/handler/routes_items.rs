use crate::dto::dto_items::{DeleteParams, EditParams, QueryParams};
use crate::model::items::ItemsModel;
use crate::response::api_response::{APIEmptyResponse, APIListResponse};
use crate::service::item_service::ItemServiceTrait;
use crate::state::item_state::ItemState;
use crate::{ERPError, ERPResult};
use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::Router;
use axum_extra::extract::WithRejection;

pub fn routes() -> Router<ItemState> {
    Router::new()
        .route("/api/items", get(api_item_list))
        .route("/api/item/edit", post(api_item_list))
        .route("/api/item/delete", post(api_item_list))
}

async fn api_item_list(
    State(state): State<ItemState>,
    WithRejection(Query(params), _): WithRejection<Query<QueryParams>, ERPError>,
) -> ERPResult<APIListResponse<ItemsModel>> {
    let items = state.item_service.get_item_list(&params).await?;
    let count = state.item_service.get_item_count(&params).await?;
    Ok(APIListResponse::new(items, count))
}

async fn api_item_edit(
    State(state): State<ItemState>,
    WithRejection(Query(params), _): WithRejection<Query<EditParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    state.item_service.edit_item(&params).await?;
    Ok(APIEmptyResponse::new())
}

async fn api_item_delete(
    State(state): State<ItemState>,
    WithRejection(Query(params), _): WithRejection<Query<DeleteParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    state.item_service.delete_item(&params).await?;
    Ok(APIEmptyResponse::new())
}
