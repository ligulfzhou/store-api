use crate::dto::dto_account::AccountDto;
use crate::dto::dto_customer::CustomerDto;
use crate::dto::dto_items::ItemsDto;
use crate::model::order::OrderModel;
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct OrderDto {
    pub id: i32,
    pub account_id: i32,
    pub account: AccountDto,
    pub customer_id: i32,
    pub customer: CustomerDto,
    pub items: Vec<ItemsDto>,
    pub create_time: NaiveDateTime,
}

impl OrderDto {
    pub fn from(
        order: OrderModel,
        account: AccountDto,
        customer: CustomerDto,
        items: Vec<ItemsDto>,
    ) -> Self {
        Self {
            id: order.id,
            account_id: order.account_id,
            account,
            customer_id: order.customer_id,
            customer,
            items,
            create_time: order.create_time,
        }
    }
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
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderParams {
    // pub account_id: i32,
    pub customer_id: i32,
    pub items: Vec<OrderItemsParams>,
    // pub order_date: String,
    // pub delivery_date: String,
}
