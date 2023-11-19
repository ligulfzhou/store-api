use crate::dto::dto_cates::{CateDto, EditParams};
use crate::response::api_response::{APIEmptyResponse, APIListResponse};
use crate::service::cates_service::CateServiceTrait;
use crate::state::cate_state::CateState;
use crate::{ERPError, ERPResult};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::WithRejection;

pub fn routes() -> Router<CateState> {
    Router::new()
        .route("/api/cates", get(api_cates_list))
        .route("/api/edit/cates", post(api_edit_cate))
        .route("/api/edit/cates2", post(api_edit_cate))
        .route("/api/delete/cates", post(api_cates_list))
}

async fn api_cates_list(State(state): State<CateState>) -> ERPResult<APIListResponse<CateDto>> {
    let cates_dto = state.cate_service.get_all_cates().await?;
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
