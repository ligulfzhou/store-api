use crate::config::database::{Database, DatabaseTrait};
use crate::model::account::AccountModel;
use std::sync::Arc;

#[derive(Clone)]
pub struct AccountRepo {
    db: Arc<Database>,
}

impl AccountRepo {
    pub fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }

    pub async fn find_user_by_account(&self, account: &str) -> Option<AccountModel> {
        sqlx::query_as!(
            AccountModel,
            "select * from accounts where account = $1",
            account
        )
        .fetch_optional(self.db.get_pool())
        .await
        .unwrap_or(None)
    }
    
}
