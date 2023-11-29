use crate::dto::dto_embryo::{EditParams, QueryParams};
use crate::dto::GenericDeleteParams;
use crate::model::embryo::EmbryoModel;
use crate::response::api_response::{APIEmptyResponse, APIListResponse};
use crate::service::embryo_service::{EmbryoService, EmbryoServiceTrait};
use crate::service::item_service::ItemServiceTrait;
use crate::state::embryo_state::EmbryoState;
use crate::{ERPError, ERPResult};
use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::WithRejection;

pub fn routes() -> Router<EmbryoState> {
    Router::new()
        .route("/api/embryos", get(api_item_list))
        .route("/api/embryo/edit", post(api_item_edit))
        .route("/api/embryo/delete", post(api_item_delete))
}

async fn api_item_list(
    State(state): State<EmbryoState>,
    WithRejection(Query(params), _): WithRejection<Query<QueryParams>, ERPError>,
) -> ERPResult<APIListResponse<EmbryoModel>> {
    let items = state.embryo_service.get_item_list(&params).await?;
    let count = state.embryo_service.get_item_count(&params).await?;
    Ok(APIListResponse::new(items, count))
}

async fn api_item_edit(
    State(state): State<EmbryoState>,
    WithRejection(Json(params), _): WithRejection<Json<EditParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    state.embryo_service.edit_item(&params).await?;
    Ok(APIEmptyResponse::new())
}

async fn api_item_delete(
    State(state): State<EmbryoState>,
    WithRejection(Json(params), _): WithRejection<Json<GenericDeleteParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    state.embryo_service.delete_item(&params).await?;
    Ok(APIEmptyResponse::new())
}
