use crate::constants::DEFAULT_PAGE_SIZE;
use crate::dto::dto_customer::CustomerDto;
use crate::model::customer::CustomerModel;
use crate::response::api_response::{APIDataResponse, APIEmptyResponse, APIListResponse};
use crate::{AppState, ERPError, ERPResult};
use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use std::sync::Arc;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/customers", get(get_customers).post(create_customer))
        .route("/api/customer/detail", get(detail_customer))
        .route("/api/customer/update", post(update_customer))
        .with_state(state)
}

#[derive(Deserialize)]
struct ListCustomerParam {
    customer_no: Option<String>,

    page: Option<i32>,
    #[serde(rename(deserialize = "pageSize"))]
    page_size: Option<i32>,
}

async fn get_customers(
    State(state): State<Arc<AppState>>,
    WithRejection(Query(param), _): WithRejection<Query<ListCustomerParam>, ERPError>,
) -> ERPResult<APIListResponse<CustomerDto>> {
    let page = param.page.unwrap_or(1);
    let page_size = param.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
    let offset = (page - 1) * page_size;
    let customer_dtos = sqlx::query_as!(
        CustomerDto,
        "select * from customers order by id desc offset $1 limit $2",
        offset as i64,
        page_size as i64
    )
    .fetch_all(&state.db)
    .await
    .map_err(ERPError::DBError)?;

    let count = sqlx::query!("select count(id) from customers")
        .fetch_one(&state.db)
        .await
        .map_err(ERPError::DBError)?
        .count
        .unwrap_or(0) as i32;

    Ok(APIListResponse::new(customer_dtos, count))
}

#[derive(Debug, Deserialize)]
struct CreateCustomerParam {
    pub customer_no: String,
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub notes: Option<String>,
}

impl CreateCustomerParam {
    fn to_sql(&self) -> String {
        format!(
            "insert into customers (customer_no, name, address, phone, notes) values ('{}', '{}', '{}', '{}', '{}')",
            self.customer_no, self.name.as_ref().unwrap_or(&"".to_string()), self.address.as_ref().unwrap_or(&"".to_string()), self.phone.as_ref().unwrap_or(&"".to_string()), self.notes.as_ref().unwrap_or(&"".to_string())
        )
    }
}

async fn create_customer(
    State(state): State<Arc<AppState>>,
    WithRejection(Json(payload), _): WithRejection<Json<CreateCustomerParam>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    let customer = sqlx::query_as!(
        CustomerModel,
        "select * from customers where customer_no = $1",
        payload.customer_no
    )
    .fetch_optional(&state.db)
    .await
    .map_err(ERPError::DBError)?;

    if customer.is_some() {
        return Err(ERPError::AlreadyExists(format!(
            "客户ID#{}已存在",
            payload.customer_no
        )));
    }

    let sql = payload.to_sql();
    state.execute_sql(&sql).await?;

    Ok(APIEmptyResponse::new())
}

#[derive(Debug, Deserialize)]
struct DetailParam {
    id: Option<i32>,
    customer_no: Option<String>,
}

async fn detail_customer(
    State(state): State<Arc<AppState>>,
    WithRejection(Query(param), _): WithRejection<Query<DetailParam>, ERPError>,
) -> ERPResult<APIDataResponse<CustomerModel>> {
    let id = param.id.unwrap_or(0);
    let customer_no = param.customer_no.as_deref().unwrap_or("");
    if id == 0 && customer_no.is_empty() {
        return Err(ERPError::ParamNeeded("id或customer_no".to_string()));
    }
    let customer = match id {
        0 => sqlx::query_as!(
            CustomerModel,
            "select * from customers where customer_no = $1",
            customer_no
        )
        .fetch_optional(&state.db)
        .await
        .map_err(ERPError::DBError)?,

        _ => sqlx::query_as!(CustomerModel, "select * from customers where id = $1", id)
            .fetch_optional(&state.db)
            .await
            .map_err(ERPError::DBError)?,
    };

    if customer.is_none() {
        return Err(ERPError::NotFound(format!(
            "客户#{:?}/{:?}",
            param.id.unwrap_or(0),
            param.customer_no.as_deref().unwrap_or("")
        )));
    }

    Ok(APIDataResponse::new(customer.unwrap()))
}

#[derive(Debug, Deserialize)]
struct UpdateCustomerParam {
    pub id: i32,
    pub customer_no: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub notes: Option<String>,
}

impl UpdateCustomerParam {
    fn to_sql(&self) -> String {
        let mut set_clauses = vec![];
        set_clauses.push(format!(
            "customer_no='{}',name='{}'",
            self.customer_no, self.name
        ));
        if let Some(address) = &self.address {
            set_clauses.push(format!("address='{}'", address))
        }
        if let Some(phone) = &self.phone {
            set_clauses.push(format!("phone='{}'", phone))
        }
        if let Some(notes) = &self.notes {
            set_clauses.push(format!("notes='{}'", notes))
        }

        format!(
            "update customers set {} where id = {};",
            set_clauses.join(","),
            self.id
        )
    }
}

async fn update_customer(
    State(state): State<Arc<AppState>>,
    WithRejection(Json(payload), _): WithRejection<Json<UpdateCustomerParam>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    let customer = sqlx::query_as!(
        CustomerModel,
        "select * from customers where id = $1",
        payload.id
    )
    .fetch_one(&state.db)
    .await
    .map_err(ERPError::DBError)?;

    if customer.customer_no != payload.customer_no
        && sqlx::query_as!(
            CustomerModel,
            "select * from customers where customer_no=$1",
            payload.customer_no
        )
        .fetch_optional(&state.db)
        .await
        .map_err(ERPError::DBError)?
        .is_some()
    {
        return Err(ERPError::Collision(format!(
            "{} 已存在",
            payload.customer_no
        )));
    }

    sqlx::query(&payload.to_sql()).execute(&state.db).await?;

    Ok(APIEmptyResponse::new())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {}
}
