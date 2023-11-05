use crate::dto::dto_items::{DeleteParams, EditParams, QueryParams};
use crate::model::items::ItemsModel;
use crate::response::api_response::APIListResponse;
use crate::state::item_state::ItemState;
use crate::ERPError;
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
    WithRejection(Query(param), _): WithRejection<Query<QueryParams>, ERPError>,
) -> Result<APIListResponse<ItemsModel>, ERPError> {
    todo!()
}

async fn api_item_edit(
    State(state): State<ItemState>,
    WithRejection(Query(param), _): WithRejection<Query<EditParams>, ERPError>,
) -> Result<APIListResponse<ItemsModel>, ERPError> {
    todo!()
}

async fn api_item_delete(
    State(state): State<ItemState>,
    WithRejection(Query(param), _): WithRejection<Query<DeleteParams>, ERPError>,
) -> Result<APIListResponse<ItemsModel>, ERPError> {
    todo!()
}
