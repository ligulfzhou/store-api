use crate::dto::dto_customer::{
    CustomerDeleteParam, CustomerDto, CustomerEditParam, CustomerSearchParam,
};
use crate::model::customer::CustomerModel;
use crate::response::api_response::{APIDataResponse, APIEmptyResponse, APIListResponse};
use crate::service::customer_service::CustomerServiceTrait;
use crate::service::settings_service::SettingsServiceTrait;
use crate::state::customer_state::CustomerState;
use crate::{ERPError, ERPResult};
use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::WithRejection;
use std::collections::HashMap;

pub fn routes() -> Router<CustomerState> {
    Router::new()
        .route("/api/customers", get(get_customers))
        .route("/api/customer/delete", get(delete_customer))
        .route("/api/customer/edit", post(edit_customer))
}

async fn get_customers(
    State(state): State<CustomerState>,
    WithRejection(Query(param), _): WithRejection<Query<CustomerSearchParam>, ERPError>,
) -> ERPResult<APIListResponse<CustomerDto>> {
    let customers = state.customer_service.get_customers(&param).await?;
    let id_to_type = state
        .settings_service
        .get_customer_types()
        .await?
        .into_iter()
        .map(|item| (item.id, item.ty_pe))
        .collect::<HashMap<i32, String>>();

    let binding = "".to_string();
    let customer_dtos = customers
        .into_iter()
        .map(|item| {
            let customer_type = id_to_type.get(&item.ty_pe).unwrap_or(&binding);
            CustomerDto::from(item, customer_type)
        })
        .collect::<Vec<_>>();

    let count = state.customer_service.get_customers_count(&param).await?;
    Ok(APIListResponse::new(customer_dtos, count))
}

async fn edit_customer(
    State(state): State<CustomerState>,
    WithRejection(Json(param), _): WithRejection<Json<CustomerEditParam>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    state.customer_service.edit_customer(&param).await?;
    Ok(APIEmptyResponse::new())
}

async fn delete_customer(
    State(state): State<CustomerState>,
    WithRejection(Query(param), _): WithRejection<Query<CustomerDeleteParam>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    state.customer_service.delete_customer(param.id).await?;
    Ok(APIEmptyResponse::new())
}
