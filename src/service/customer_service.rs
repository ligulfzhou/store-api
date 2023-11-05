use crate::config::database::{Database, DatabaseTrait};
use crate::dto::dto_customer::{CustomerEditParam, CustomerSearchParam};
use crate::model::customer::CustomerModel;
use crate::ERPResult;
use async_trait::async_trait;
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

    async fn edit_customer(&self, param: &CustomerEditParam) -> ERPResult<()>;

    async fn delete_customer(&self, id: i32) -> ERPResult<()>;
}

#[async_trait]
impl CustomerServiceTrait for CustomerService {
    fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }

    async fn get_customers(&self, param: &CustomerSearchParam) -> ERPResult<Vec<CustomerModel>> {
        let sql = param.to_pagination_sql();
        tracing::info!("get_customers: to_pagination_sql: {:?}", sql);
        let customers = sqlx::query_as::<_, CustomerModel>(&sql)
            .fetch_all(self.db.get_pool())
            .await?;

        Ok(customers)
    }

    async fn get_customers_count(&self, param: &CustomerSearchParam) -> ERPResult<i32> {
        let sql = param.to_count_sql();
        tracing::info!("get_customers: to_count_sql: {:?}", sql);
        let count = sqlx::query_as::<_, (i64,)>(&sql)
            .fetch_one(self.db.get_pool())
            .await?
            .0 as i32;

        Ok(count)
    }

    async fn edit_customer(&self, param: &CustomerEditParam) -> ERPResult<()> {
        let id = param.id.unwrap_or(0);
        match id {
            0 => {
                sqlx::query!(
                    r#"
                    insert into customers (customer_no, ty_pe, name, head, address, 
                        email, birthday, qq, phone, notes)
                    values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10);
                    "#,
                    param.customer_no,
                    param.ty_pe,
                    param.name,
                    param.head,
                    param.address,
                    param.email,
                    param.birthday,
                    param.qq,
                    param.phone,
                    param.notes,
                )
                .execute(self.db.get_pool())
                .await?;
            }
            _ => {
                sqlx::query!(
                    r#"
                    update customers set customer_no=$1, ty_pe=$2, name=$3, 
                        head=$4, address=$5, email=$6, birthday=$7, 
                        qq=$8, phone=$9, notes=$10 
                    where id=$11"#,
                    param.customer_no,
                    param.ty_pe,
                    param.name,
                    param.head,
                    param.address,
                    param.email,
                    param.birthday,
                    param.qq,
                    param.phone,
                    param.notes,
                    param.id,
                )
                .execute(self.db.get_pool())
                .await?;
            }
        };

        Ok(())
    }

    async fn delete_customer(&self, id: i32) -> ERPResult<()> {
        sqlx::query!("delete from customers where id = $1", id)
            .execute(self.db.get_pool())
            .await?;
        Ok(())
    }
}
