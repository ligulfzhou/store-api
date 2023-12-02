use crate::config::database::{Database, DatabaseTrait};
use crate::model::account::AccountModel;
use crate::ERPResult;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct AccountService {
    pub db: Arc<Database>,
}

#[async_trait]
pub trait AccountServiceTrait {
    fn new(db: &Arc<Database>) -> Self;
    async fn get_accounts(&self, account_ids: &[i32]) -> ERPResult<Vec<AccountModel>>;
}

#[async_trait]
impl AccountServiceTrait for AccountService {
    fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }

    async fn get_accounts(&self, account_ids: &[i32]) -> ERPResult<Vec<AccountModel>> {
        let accounts = sqlx::query_as!(
            AccountModel,
            "select * from accounts where id = any($1)",
            &account_ids
        )
        .fetch_all(self.db.get_pool())
        .await?;

        Ok(accounts)
    }
}
