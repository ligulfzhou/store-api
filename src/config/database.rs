use crate::config::parameter;
use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

pub struct Database {
    pool: Pool<Postgres>,
}

#[async_trait]
pub trait DatabaseTrait {
    async fn init() -> Result<Self, Error>
    where
        Self: Sized;
    fn get_pool(&self) -> &Pool<Postgres>;
}

#[async_trait]
impl DatabaseTrait for Database {
    async fn init() -> Result<Self, Error> {
        let database_url = parameter::get("DATABASE_URL");
        tracing::info!("{database_url}");

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await?;

        Ok(Self { pool })
    }

    fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
