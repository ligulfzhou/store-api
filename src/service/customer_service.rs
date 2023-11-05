use crate::config::database::{Database, DatabaseTrait};
use crate::dto::dto_customer::CustomerSearchParam;
use crate::model::customer::CustomerModel;
use crate::ERPResult;
use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct CustomerService {
    pub db: Arc<Database>,
}

#[async_trait]
pub trait CustomerServiceTrait {
    fn new(db: &Arc<Database>) -> Self;

    async fn get_customers(&self, param: &CustomerSearchParam) -> ERPResult<Vec<CustomerModel>>;

    async fn get_customers_count(&self, param: &CustomerSearchParam) -> ERPResult<i32>;
}

#[async_trait]
impl CustomerServiceTrait for CustomerService {
    fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }

    async fn get_customers(&self, param: &CustomerSearchParam) -> ERPResult<Vec<CustomerModel>> {
        let sql = param.to_pagination_sql();
        let customers = sqlx::query_as::<_, CustomerModel>(&sql)
            .fetch_all(self.db.get_pool())
            .await?;

        Ok(customers)
    }

    async fn get_customers_count(&self, param: &CustomerSearchParam) -> ERPResult<i32> {
        let sql = param.to_count_sql();
        let count = sqlx::query_as::<_, (i64,)>(&sql)
            .fetch_one(self.db.get_pool())
            .await?
            .0 as i32;

        Ok(count)
    }
}
