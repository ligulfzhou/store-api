use crate::model::settings::ColorSettingsModel;
use crate::response::api_response::APIListResponse;
use crate::service::settings_service::SettingsServiceTrait;
use crate::state::settings_state::SettingsState;
use crate::{ERPError, ERPResult};
use axum::{
    extract::State,
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<SettingsState> {
    Router::new().route("/api/settings/color/value", get(api_get_color_values))
}

async fn api_get_color_values(
    State(state): State<SettingsState>,
) -> ERPResult<APIListResponse<ColorSettingsModel>> {
    tracing::info!("->> {:<12}, api_get_color_values", "handler");

    let color_values = state.settings_service.get_all_color_to_values().await?;
    let len = color_values.len() as i32;

    Ok(APIListResponse::new(color_values, len))
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
