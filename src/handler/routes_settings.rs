use crate::dto::dto_settings::{
    ColorEditParams, CustomerTypeEditParams, GlobalSettingsUpdateParams,
};
use crate::dto::GenericDeleteParams;
use crate::model::settings::{ColorSettingsModel, CustomerTypeModel, GlobalSettingsModel};
use crate::response::api_response::{APIDataResponse, APIEmptyResponse, APIListResponse};
use crate::service::settings_service::SettingsServiceTrait;
use crate::state::settings_state::SettingsState;
use crate::{ERPError, ERPResult};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::WithRejection;

pub fn routes() -> Router<SettingsState> {
    Router::new()
        .route("/api/settings/color/value", get(api_get_color_values))
        .route(
            "/api/settings/edit/color/value",
            post(api_edit_color_values),
        )
        .route(
            "/api/settings/delete/color/value",
            post(api_delete_color_values),
        )
        .route("/api/settings/global", get(api_get_global_settings))
        .route(
            "/api/settings/global/update",
            post(api_update_global_settings),
        )
        .route("/api/settings/customer/types", get(api_get_customer_types))
        .route(
            "/api/settings/edit/customer/type",
            post(api_edit_customer_type),
        )
        .route(
            "/api/settings/delete/customer/type",
            post(api_delete_customer_type),
        )
}

async fn api_get_color_values(
    State(state): State<SettingsState>,
) -> ERPResult<APIListResponse<ColorSettingsModel>> {
    tracing::info!("->> {:<12}, api_get_color_values", "handler");

    let color_values = state.settings_service.get_all_color_to_values().await?;
    let len = color_values.len() as i32;

    Ok(APIListResponse::new(color_values, len))
}

async fn api_edit_color_values(
    State(state): State<SettingsState>,
    WithRejection(Json(params), _): WithRejection<Json<ColorEditParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    tracing::info!("->> {:<12}, api_get_color_values", "handler");

    state.settings_service.edit_color_to_value(&params).await?;

    Ok(APIEmptyResponse::new())
}

async fn api_delete_color_values(
    State(state): State<SettingsState>,
    WithRejection(Json(params), _): WithRejection<Json<GenericDeleteParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    tracing::info!("->> {:<12}, delete_get_color_values", "handler");

    state
        .settings_service
        .delete_color_to_value(&params)
        .await?;

    Ok(APIEmptyResponse::new())
}

async fn api_get_global_settings(
    State(state): State<SettingsState>,
) -> ERPResult<APIDataResponse<GlobalSettingsModel>> {
    tracing::info!("->> {:<12}, api_get_global_settings", "handler");

    let gs = state.settings_service.get_global_settings().await?;

    Ok(APIDataResponse::new(gs))
}

async fn api_update_global_settings(
    State(state): State<SettingsState>,
    WithRejection(Json(params), _): WithRejection<Json<GlobalSettingsUpdateParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    tracing::info!("->> {:<12}, api_update_global_settings", "handler");

    state
        .settings_service
        .update_global_settings(&params)
        .await?;

    Ok(APIEmptyResponse::new())
}

async fn api_get_customer_types(
    State(state): State<SettingsState>,
) -> ERPResult<APIListResponse<CustomerTypeModel>> {
    tracing::info!("->> {:<12}, api_get_customer_types", "handler");

    let ccustomer_types = state.settings_service.get_customer_types().await?;
    let len = ccustomer_types.len() as i32;

    Ok(APIListResponse::new(ccustomer_types, len))
}

async fn api_edit_customer_type(
    State(state): State<SettingsState>,
    WithRejection(Json(params), _): WithRejection<Json<CustomerTypeEditParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    tracing::info!("->> {:<12}, api_edit_customer_type", "handler");

    state.settings_service.edit_customer_type(&params).await?;

    Ok(APIEmptyResponse::new())
}

async fn api_delete_customer_type(
    State(state): State<SettingsState>,
    WithRejection(Json(params), _): WithRejection<Json<GenericDeleteParams>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    tracing::info!("->> {:<12}, api_delete_customer_type", "handler");

    state.settings_service.delete_customer_type(&params).await?;

    Ok(APIEmptyResponse::new())
}
