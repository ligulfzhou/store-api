use crate::dto::dto_cates::CateDto;
use crate::response::api_response::APIListResponse;
use crate::service::cates_service::CateServiceTrait;
use crate::state::cate_state::CateState;
use crate::ERPResult;
use axum::extract::State;
use axum::routing::{get, post};
use axum::Router;

pub fn routes() -> Router<CateState> {
    Router::new()
        .route("/api/cates", get(api_cates_list))
        .route("/api/save/cates", post(api_cates_list))
        .route("/api/update/cates", post(api_cates_list))
        .route("/api/extract/cates", post(api_cates_list))
}

async fn api_cates_list(State(state): State<CateState>) -> ERPResult<APIListResponse<CateDto>> {
    let cates_dto = state.cate_service.get_all_cates().await?;
    let count = cates_dto.len() as i32;
    Ok(APIListResponse::new(cates_dto, count))
}

#[derive(Debug, Deserialize)]
pub struct CateParam {
    pub id: i32,             // SERIAL
    pub index: i32,          // 顺序
    pub name: String,        // 类名
    pub cate_type: i32,      // 大类小类， 0 大类， 1小类，再变大，则更小
    pub parent_name: String, // 父类
}

async fn api_save_cates() -> ERPResult<()> {
    todo!()
}

// #[cfg(test)]
// mod tests {
//     use crate::handler::routes_login::LoginPayload;
//
//     #[tokio::test]
//     async fn test() -> anyhow::Result<()> {
//         let param = LoginPayload {
//             account: "test".to_string(),
//             password: "test".to_string(),
//         };
//         let client = httpc_test::new_client("http://localhost:9100")?;
//         client
//             .do_post("/api/login", serde_json::json!(param))
//             .await?
//             .print()
//             .await?;
//
//         client.do_get("/api/account/info").await?.print().await?;
//         Ok(())
//     }
// }
