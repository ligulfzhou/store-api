use crate::dto::dto_account::AccountDto;
use crate::handler::ListParamToSQLTrait;
use crate::model::items::ItemsModel;
use crate::response::api_response::{APIDataResponse, APIListResponse};
use crate::{AppState, ERPError};
use axum::body::HttpBody;
use axum::extract::{Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use sqlx::{Execute, Postgres, QueryBuilder};
use std::sync::Arc;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/items", get(api_item_list))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct ItemListParam {
    pub cates1_id: i32,
    pub cates2_id: i32,
    pub has_storage: i32, // 0, 1(有库存), 2(无库存) // todo
    pub sorter_field: Option<String>,
    pub sorter_order: Option<String>,
}

async fn api_item_list(
    State(state): State<Arc<AppState>>,
    WithRejection(Query(param), _): WithRejection<Query<ItemListParam>, ERPError>,
) -> Result<APIListResponse<ItemsModel>, ERPError> {
    let items = ItemsModel::get_list(&state.db, &param).await?;
    let count = ItemsModel::get_count(&state.db, &param).await?;

    Ok(APIListResponse::new(items, count))
}

#[cfg(test)]
mod tests {
    use crate::handler::routes_login::LoginPayload;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
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
