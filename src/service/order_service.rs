use crate::config::database::{Database, DatabaseTrait};
use crate::dto::dto_orders::{CreateOrderParams, OrderDto, OrderItemsParams};
use crate::model::order::OrderItemModel;
use crate::ERPResult;
use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;

#[derive(Clone)]
pub struct OrderService {
    pub db: Arc<Database>,
}

#[async_trait]
pub trait OrderServiceTrait {
    fn new(db: &Arc<Database>) -> Self;

    async fn create_order(&self, params: &CreateOrderParams) -> ERPResult<i32>;

    async fn insert_order_items(
        &self,
        items: &[OrderItemsParams],
        order_id: i32,
    ) -> ERPResult<Vec<OrderItemModel>>;

    async fn get_order(&self, order_id: i32) -> ERPResult<OrderDto>;
}

#[async_trait]
impl OrderServiceTrait for OrderService {
    fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }

    async fn create_order(&self, params: &CreateOrderParams) -> ERPResult<i32> {
        let order = sqlx::query!(
            "insert into orders(account_id, customer_id) values ($1, $2) returning *",
            params.account_id,
            params.customer_id
        )
        .fetch_one(self.db.get_pool())
        .await?;

        let order_items = self.insert_order_items(&params.items, order.id).await?;

        Ok(order.id)
    }

    async fn insert_order_items(
        &self,
        items: &[OrderItemsParams],
        order_id: i32,
    ) -> ERPResult<Vec<OrderItemModel>> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "insert into order_items (order_id, item_id, count, price, origin_price, discount) ",
        );

        query_builder.push_values(items, |mut b, item| {
            b.push_bind(order_id)
                .push_bind(item.item_id)
                .push_bind(item.count)
                .push_bind(item.price)
                .push_bind(item.origin_price)
                .push_bind(item.discount);
        });
        query_builder.push(" returning *;");

        let res = query_builder
            .build_query_as::<OrderItemModel>()
            .fetch_all(self.db.get_pool())
            .await?;

        Ok(res)
    }

    async fn get_order(&self, order_id: i32) -> ERPResult<OrderDto> {
        todo!()
    }
}
