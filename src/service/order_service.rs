use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub struct OrderService {}

pub trait OrderServiceTrait {
    fn new(pool: Arc<Pool<Postgres>>) -> Self;
    fn find_by_order_id();
}
