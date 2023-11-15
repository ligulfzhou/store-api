use crate::dto::dto_customer::{CustomerDeleteParam, CustomerEditParam, CustomerSearchParam};
use crate::model::customer::CustomerModel;
use crate::response::api_response::{APIDataResponse, APIEmptyResponse, APIListResponse};
use crate::service::customer_service::CustomerServiceTrait;
use crate::state::customer_state::CustomerState;
use crate::{ERPError, ERPResult};
use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::WithRejection;

pub fn routes() -> Router<CustomerState> {
    Router::new()
        .route("/api/customers", get(get_customers))
        .route("/api/customer/delete", get(delete_customer))
        .route("/api/customer/edit", post(edit_customer))
}

async fn get_customers(
    State(state): State<CustomerState>,
    WithRejection(Query(param), _): WithRejection<Query<CustomerSearchParam>, ERPError>,
) -> ERPResult<APIListResponse<CustomerModel>> {
    let customers = state.customer_service.get_customers(&param).await?;
    let count = state.customer_service.get_customers_count(&param).await?;
    // todo: convert model => dto
    Ok(APIListResponse::new(customers, count))
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
