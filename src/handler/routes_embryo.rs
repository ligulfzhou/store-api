use crate::dto::dto_account::AccountDto;
use crate::dto::dto_embryo::{EditParams, EmbryoDto, InoutParams, QueryParams};
use crate::dto::GenericDeleteParams;
use crate::response::api_response::{APIEmptyResponse, APIListResponse};
use crate::service::embryo_service::EmbryoServiceTrait;
use crate::state::embryo_state::EmbryoState;
use crate::{ERPError, ERPResult};
use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::WithRejection;

pub fn routes() -> Router<EmbryoState> {
    Router::new()
        .route("/api/embryos", get(api_item_list))
        .route("/api/embryo/edit", post(api_item_edit))
        .route("/api/embryo/delete", post(api_item_delete))
        .route("/api/embryo/inout", post(api_item_inout))
}

async fn api_item_list(
    State(state): State<EmbryoState>,
    WithRejection(Query(params), _): WithRejection<Query<QueryParams>, ERPError>,
) -> ERPResult<APIListResponse<EmbryoDto>> {
    tracing::info!("api_item_list : /api/embryos");

    let items = state.embryo_service.get_item_list(&params).await?;
    let embryo_dtos = state.embryo_service.embryos_to_embryo_dtos(items).await?;
    let count = state.embryo_service.get_item_count(&params).await?;
    Ok(APIListResponse::new(embryo_dtos, count))
}

async fn api_item_edit(
    State(state): State<EmbryoState>,
    WithRejection(Json(params), _): WithRejection<Json<EditParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    tracing::info!("api_item_edit : /api/embryo/edit");

    state.embryo_service.edit_item(&params).await?;
    Ok(APIEmptyResponse::new())
}

async fn api_item_delete(
    State(state): State<EmbryoState>,
    WithRejection(Json(params), _): WithRejection<Json<GenericDeleteParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    tracing::info!("api_item_delete : /api/embryo/delete");

    state.embryo_service.delete_item(&params).await?;
    Ok(APIEmptyResponse::new())
}

async fn api_item_inout(
    State(state): State<EmbryoState>,
    Extension(account): Extension<AccountDto>,
    WithRejection(Json(params), _): WithRejection<Json<InoutParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    tracing::info!("api_item_inout : /api/embryo/inout");
    state
        .embryo_service
        .add_item_inout(&params, account.id)
        .await?;
    Ok(APIEmptyResponse::new())
}
