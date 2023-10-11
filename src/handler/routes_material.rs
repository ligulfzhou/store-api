use crate::constants::DEFAULT_PAGE_SIZE;
use crate::handler::ListParamToSQLTrait;
use crate::model::order::{OrderItemMaterialModel, OrderItemModel};
use crate::response::api_response::{APIEmptyResponse, APIListResponse};
use crate::{AppState, ERPError, ERPResult};
use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/order/item/materials",
            get(get_order_item_materials).post(add_order_item_materials),
        )
        .with_state(state)
}

#[derive(Debug, Deserialize, Serialize)]
struct ListOrderItemMaterialsParam {
    pub order_id: Option<i32>,
    pub order_item_id: i32,
    pub name: Option<String>,
    pub color: Option<String>,
    pub page: Option<i32>,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: Option<i32>,
}

impl ListParamToSQLTrait for ListOrderItemMaterialsParam {
    fn to_pagination_sql(&self) -> String {
        let mut sql = "select * from order_item_materials".to_string();
        let mut where_clauses = vec![];
        if let Some(order_id) = self.order_id {
            where_clauses.push(format!("order_id={}", order_id));
        }
        where_clauses.push(format!("order_item_id={}", self.order_item_id));
        if self.name.is_some() && !self.name.as_ref().unwrap().is_empty() {
            where_clauses.push(format!("name='{}'", self.name.as_ref().unwrap()));
        }
        if self.color.is_some() && !self.color.as_ref().unwrap().is_empty() {
            where_clauses.push(format!("color='{}'", self.color.as_ref().unwrap()));
        }
        sql.push_str(" where ");
        sql.push_str(&where_clauses.join(" and "));

        let page = self.page.unwrap_or(1);
        let page_size = self.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        sql.push_str(&format!(
            " order by id desc limit {page} offset {page_size};"
        ));

        tracing::info!("{sql}");
        sql
    }

    fn to_count_sql(&self) -> String {
        let mut sql = "select count(1) from order_item_materials".to_string();
        let mut where_clauses = vec![];
        if let Some(order_id) = self.order_id {
            where_clauses.push(format!("order_id={}", order_id));
        }
        where_clauses.push(format!("order_item_id={}", self.order_item_id));
        if self.name.is_some() && !self.name.as_ref().unwrap().is_empty() {
            where_clauses.push(format!("name='{}'", self.name.as_ref().unwrap()));
        }
        if self.color.is_some() && !self.color.as_ref().unwrap().is_empty() {
            where_clauses.push(format!("color='{}'", self.color.as_ref().unwrap()));
        }
        sql.push_str(" where ");
        sql.push_str(&where_clauses.join(" and "));
        sql.push(';');

        tracing::info!("{sql}");
        sql
    }
}

async fn get_order_item_materials(
    State(state): State<Arc<AppState>>,
    WithRejection(Query(param), _): WithRejection<Query<ListOrderItemMaterialsParam>, ERPError>,
) -> ERPResult<APIListResponse<OrderItemMaterialModel>> {
    let materials = sqlx::query_as::<_, OrderItemMaterialModel>(&param.to_pagination_sql())
        .fetch_all(&state.db)
        .await
        .map_err(ERPError::DBError)?;

    let total: (i64,) = sqlx::query_as(&param.to_count_sql())
        .fetch_one(&state.db)
        .await
        .map_err(ERPError::DBError)?;

    Ok(APIListResponse::new(materials, total.0 as i32))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CreateOrderItemMaterialParam {
    pub order_id: i32,
    pub order_item_id: i32,
    pub name: String,
    pub color: String,
    // material_id   integer, -- 材料ID  (暂时先不用)
    pub single: Option<i32>,   //  integer, -- 单数      ？小数
    pub count: i32,            //  integer, -- 数量      ？小数
    pub total: Option<i32>,    //  integer, -- 总数(米)  ? 小数
    pub stock: Option<i32>,    //  integer, -- 库存 ?
    pub debt: Option<i32>,     //  integer, -- 欠数
    pub notes: Option<String>, //  text     -- 备注
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CreateOrderItemMaterialsParam {
    order_item_id: i32,
    materials: Vec<CreateOrderItemMaterialParam>,
}

impl CreateOrderItemMaterialsParam {
    fn to_sql(&self) -> String {
        let values = self
            .materials
            .iter()
            .map(|material| {
                format!(
                    "({}, {}, '{}', '{}', {:?}, {}, {:?}, {:?}, {:?}, '{:?}')",
                    material.order_id,
                    material.order_item_id,
                    material.name,
                    material.color,
                    material.single,
                    material.count,
                    material.total,
                    material.stock,
                    material.debt,
                    material.notes
                )
            })
            .collect::<Vec<String>>()
            .join(",");

        format!("insert into order_item_materials (order_id, order_item_id, name, color, single, count, total, stock, debt, notes) values {};", values)
    }
}

async fn add_order_item_materials(
    State(state): State<Arc<AppState>>,
    WithRejection(Json(payload), _): WithRejection<Json<CreateOrderItemMaterialsParam>, ERPError>,
) -> ERPResult<APIEmptyResponse> {
    // checking material is empty
    if payload.materials.is_empty() {
        return Err(ERPError::ParamNeeded("materials".to_string()));
    }

    // checking material
    let existings = sqlx::query_as::<_, OrderItemMaterialModel>(&format!(
        "select * from order_item_materials where order_item_id={};",
        payload.order_item_id
    ))
    .fetch_all(&state.db)
    .await
    .map_err(ERPError::DBError)?;

    // if already have some material, then check against it.
    if !existings.is_empty() {
        let existing_name_color_tuples: Vec<(String, String)> = existings
            .iter()
            .map(|material| (material.name.clone(), material.color.clone()))
            .collect();

        let duplicates = payload
            .materials
            .iter()
            .filter(|&material| {
                existing_name_color_tuples
                    .contains(&(material.name.clone(), material.color.clone()))
            })
            .map(|material| format!("({}-{})", material.name, material.color))
            .collect::<Vec<String>>();

        if !duplicates.is_empty() {
            return Err(ERPError::AlreadyExists(duplicates.join(",")));
        }
    }

    state.execute_sql(&payload.to_sql()).await?;

    Ok(APIEmptyResponse::new())
}

#[derive(Debug, Deserialize)]
struct UpdateOrderItemMaterialParam {
    id: i32,
    order_id: i32,
    sku_id: i32,
    package_card: Option<String>,
    package_card_des: Option<String>,
    count: i32,
    unit: Option<String>,
    unit_price: Option<i32>,
    total_price: Option<i32>,
    notes: Option<String>,
}

impl UpdateOrderItemMaterialParam {
    fn to_sql(&self) -> String {
        let mut sql = format!(
            "update order_items set order_id={},sku_id={},count={}",
            self.order_id, self.sku_id, self.count
        );
        if let Some(package_card) = &self.package_card {
            sql.push_str(&format!(",package_card='{}'", package_card));
        }
        if let Some(package_card_des) = &self.package_card_des {
            sql.push_str(&format!(",package_card_des='{}'", package_card_des));
        }
        if let Some(unit) = &self.unit {
            sql.push_str(&format!(",unit='{}'", unit));
        }
        if let Some(unit_price) = &self.unit_price {
            sql.push_str(&format!(",unit_price={}", unit_price));
        }
        if let Some(total_price) = &self.total_price {
            sql.push_str(&format!(",total_price='{}'", total_price));
        }
        if let Some(notes) = &self.notes {
            sql.push_str(&format!(",notes={}", notes));
        }

        sql
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tokio;

    #[tokio::test]
    async fn test_create_order() -> Result<()> {
        Ok(())
    }
}
