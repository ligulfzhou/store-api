use crate::{ERPError, ERPResult};
use sqlx::{Pool, Postgres};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct CustomerModel {
    pub id: i32,
    pub customer_no: String,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub notes: String,
}

impl CustomerModel {
    pub async fn get_customer_with_customer_no(
        db: &Pool<Postgres>,
        customer_no: &str,
    ) -> ERPResult<CustomerModel> {
        let customer = sqlx::query_as!(
            CustomerModel,
            "select * from customers where customer_no=$1",
            customer_no
        )
        .fetch_one(db)
        .await
        .map_err(ERPError::DBError)?;

        Ok(customer)
    }
}
