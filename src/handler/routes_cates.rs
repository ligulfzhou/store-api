use crate::dto::dto_account::AccountDto;
use crate::dto::dto_cates::{CateDto, EditParams, SubCatesParams};
use crate::dto::GenericDeleteParams;
use crate::response::api_response::{APIEmptyResponse, APIListResponse};
use crate::service::cates_service::CateServiceTrait;
use crate::state::cate_state::CateState;
use crate::{ERPError, ERPResult};
use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::WithRejection;

pub fn routes() -> Router<CateState> {
    Router::new()
        .route("/api/cates", get(api_cates_list))
        .route("/api/cates/sub/cates", get(api_sub_cates_list))
        .route("/api/edit/cates", post(api_edit_cate))
        .route("/api/edit/cates2", post(api_edit_cate))
        .route("/api/delete/cates", post(api_delete_cate))
}

async fn api_cates_list(
    State(state): State<CateState>,
    Extension(_account): Extension<AccountDto>,
) -> ERPResult<APIListResponse<CateDto>> {
    let cates_dto = state.cate_service.get_all_cates().await?;
    let count = cates_dto.len() as i32;
    Ok(APIListResponse::new(cates_dto, count))
}

async fn api_sub_cates_list(
    State(state): State<CateState>,
    WithRejection(Query(params), _): WithRejection<Query<SubCatesParams>, ERPError>,
) -> ERPResult<APIListResponse<CateDto>> {
    let cates_dto = state.cate_service.get_sub_cates_of(params.id).await?;
    let count = cates_dto.len() as i32;
    Ok(APIListResponse::new(cates_dto, count))
}

async fn api_edit_cate(
    State(state): State<CateState>,
    WithRejection(Json(params), _): WithRejection<Json<EditParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    state.cate_service.edit_cates(&params).await?;

    Ok(APIEmptyResponse::new())
}

async fn api_delete_cate(
    State(state): State<CateState>,
    WithRejection(Json(params), _): WithRejection<Json<GenericDeleteParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    tracing::info!("->> {:<12}, delete_get_color_values", "handler");

    state.cate_service.delete_cate(&params).await?;

    Ok(APIEmptyResponse::new())
}

#[cfg(test)]
mod tests {
    use crate::handler::routes_login::LoginPayload;
    use crate::ERPResult;

    #[tokio::test]
    async fn test() -> ERPResult<()> {
        let param = LoginPayload {
            account: "test".to_string(),
            password: "test".to_string(),
        };
        let client = httpc_test::new_client("http://localhost:9100")?;
        client
            .do_post("/api/login", serde_json::json!(param))
            .await?
            .print()
            .await?;

        client.do_get("/api/account/info").await?.print().await?;
        Ok(())
    }
}
