use crate::model::items::ItemsModel;
use crate::response::api_response::APIListResponse;
use crate::state::cate_state::CateState;
use crate::ERPResult;
use axum::extract::State;
use axum::routing::{get, post};
use axum::Router;

pub fn routes() -> Router<CateState> {
    Router::new()
        .route("/api/items", get(api_cates_list))
        .route("/api/save/cates", post(api_cates_list))
}

async fn api_cates_list(State(state): State<CateState>) -> ERPResult<APIListResponse<ItemsModel>> {
    // Ok(APIListResponse::new(items, count))

    todo!()
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
