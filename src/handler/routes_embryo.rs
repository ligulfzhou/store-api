use crate::constants::DEFAULT_PAGE_SIZE;
use crate::dto::dto_account::AccountDto;
use crate::dto::dto_embryo::{
    EditParams, EmbryoDto, EmbryoInOutBucketDto, EmbryoInOutDto, InoutBucketParams,
    InoutListOfBucketParams, InoutListParams, InoutParams, QueryParams,
};
use crate::dto::GenericDeleteParams;
use crate::repository::embryo_repository::EmbryoRepositoryTrait;
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
        .route("/api/embryo/inout/list", get(api_inout_list))
        .route("/api/embryo/inout/group/list", get(api_inout_group_list)) // 出入库列表
        .route(
            "/api/embryo/inout/list/of/bucket",
            get(api_inout_list_of_bucket),
        ) // 出入库列表
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

async fn api_inout_list(
    State(state): State<EmbryoState>,
    Extension(account): Extension<AccountDto>,
    WithRejection(Query(params), _): WithRejection<Query<InoutListParams>, ERPError>,
) -> ERPResult<APIListResponse<EmbryoInOutDto>> {
    tracing::info!("api_item_list : /api/embryo/inout/list");

    let items = state
        .embryo_service
        .embryo_repo
        .inout_list_of_embryo(
            params.embryo_id,
            &account.name,
            params.page.unwrap_or(1),
            params.page_size.unwrap_or(DEFAULT_PAGE_SIZE),
        )
        .await?;

    let count = state
        .embryo_service
        .embryo_repo
        .inout_list_of_embryo_count(params.embryo_id)
        .await?;
    Ok(APIListResponse::new(items, count))
}

async fn api_inout_group_list(
    State(state): State<EmbryoState>,
    // Extension(account): Extension<AccountDto>,
    WithRejection(Query(params), _): WithRejection<Query<InoutBucketParams>, ERPError>,
) -> ERPResult<APIListResponse<EmbryoInOutBucketDto>> {
    tracing::info!("api_item_list : /api/embryo/inout/group/list");

    let buckets = state.embryo_service.inout_bucket_list(&params).await?;
    let count = state.embryo_service.inout_bucket_count(&params).await?;

    Ok(APIListResponse::new(buckets, count))
}

async fn api_inout_list_of_bucket(
    State(state): State<EmbryoState>,
    WithRejection(Query(params), _): WithRejection<Query<InoutListOfBucketParams>, ERPError>,
) -> ERPResult<APIListResponse<EmbryoInOutDto>> {
    tracing::info!("api_item_list : /api/embryo/inout/list/of/bucket");

    if params.bucket_id == 0 {
        return Ok(APIListResponse::new(vec![], 0));
    }

    let items = state.embryo_service.inout_list_of_bucket(&params).await?;
    let count = state.embryo_service.inout_count_of_bucket(&params).await?;

    Ok(APIListResponse::new(items, count))
}
