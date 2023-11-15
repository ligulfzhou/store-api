use crate::config::database::{Database, DatabaseTrait};
use crate::dto::dto_customer::{CustomerEditParam, CustomerSearchParam};
use crate::model::customer::CustomerModel;
use crate::{ERPError, ERPResult};
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
        let existing = sqlx::query_as!(
            CustomerModel,
            "select * from customers where name = $1",
            param.name
        )
        .fetch_all(self.db.get_pool())
        .await?;

        let id = param.id.unwrap_or(0);
        match id {
            0 => {
                if !existing
                    .iter()
                    .filter(|item| item.name == param.name)
                    .collect::<Vec<&CustomerModel>>()
                    .is_empty()
                {
                    return Err(ERPError::AlreadyExists(format!(
                        "名字为{}已经存在",
                        param.name
                    )));
                }

                sqlx::query!(
                    r#"
                    insert into customers (ty_pe, name, head, address, 
                        email, birthday, phone, notes)
                    values ($1, $2, $3, $4, $5, $6, $7, $8);
                    "#,
                    param.ty_pe,
                    param.name,
                    param.head,
                    param.address,
                    param.email,
                    param.birthday,
                    param.phone,
                    param.notes,
                )
                .execute(self.db.get_pool())
                .await?;
            }
            _ => {
                if !existing
                    .iter()
                    .filter(|item| item.name == param.name && item.id != id)
                    .collect::<Vec<&CustomerModel>>()
                    .is_empty()
                {
                    return Err(ERPError::AlreadyExists(format!(
                        "名字为{}已经存在",
                        param.name
                    )));
                }
                sqlx::query!(
                    r#"
                    update customers set notes=$1, ty_pe=$2, name=$3, 
                        head=$4, address=$5, email=$6, birthday=$7, 
                        phone=$8
                    where id=$9"#,
                    param.notes,
                    param.ty_pe,
                    param.name,
                    param.head,
                    param.address,
                    param.email,
                    param.birthday,
                    param.phone,
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
