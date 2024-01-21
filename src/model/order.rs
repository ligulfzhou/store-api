use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Serialize, Clone, sqlx::FromRow)]
pub struct OrderModel {
    pub id: i32,
    pub order_no: String,
    pub tp: i32,
    pub account_id: i32,
    pub customer_id: i32,
    pub order_date: NaiveDate,
    pub delivery_date: NaiveDate,
    pub create_time: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderItemModel {
    pub id: i32,
    pub order_id: i32,
    pub index: i32,
    pub item_id: i32,
    pub count: i32,
    pub origin_price: i32,
    pub price: i32,
    pub total_price: i32,
    pub discount: i32,
    pub create_time: NaiveDateTime,
}
