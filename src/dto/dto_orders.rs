use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct OrderDto {
    pub id: i32,
    pub order_no: String,
    pub tp: i32,
    pub account_id: i32,
    pub account: String,
    pub customer_id: i32,
    pub customer: String,
    pub create_time: NaiveDateTime,
    pub order_date: NaiveDate,
    pub delivery_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderItemDto {
    pub id: i32,
    pub order_id: i32,
    pub index: i32,
    pub item_id: i32,
    pub item_images: Vec<String>,
    pub count: i32,
    pub origin_price: i32,
    pub price: i32,
    pub total_price: i32,
    pub discount: i32,
    pub create_time: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct OrderDetailDto {
    pub order: OrderDto,
    pub items: Vec<OrderItemDto>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct OrderInListDto {
    pub id: i32,
    pub account_id: i32,
    pub account: String,
    pub customer_id: i32,
    pub customer: String,
    pub item_images: Vec<String>,
    pub create_time: NaiveDateTime,
    pub total: i32,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct OrderDetailQueryParams {
    pub order_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub customer_id: i32,
    pub account_id: i32,
    pub create_time_st: String,
    pub create_time_ed: String,

    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

impl QueryParams {
    pub fn is_empty(&self) -> bool {
        if self.customer_id != 0 {
            return false;
        }
        if self.account_id != 0 {
            return false;
        }
        if !self.create_time_ed.is_empty() && !self.create_time_st.is_empty() {
            return false;
        }
        true
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderItemsParams {
    pub item_id: i32,
    pub count: i32,
    // pub price: i32,
    // pub origin_price: i32,
    pub discount: i32,
    pub discount_price: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderParams {
    // pub account_id: i32,
    pub customer_id: i32,
    pub items: Vec<OrderItemsParams>,
    // pub order_date: String,
    // pub delivery_date: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteOrderParams {
    pub order_id: i32,
}
