use crate::config::database::Database;
use crate::repository::account_repository::AccountRepo;
use crate::service::account_service::AccountService;
use crate::service::account_service::AccountServiceTrait;
use std::sync::Arc;

#[derive(Clone)]
pub struct AccountState {
    pub account_service: AccountService,
    pub account_repo: AccountRepo,
}

impl AccountState {
    pub fn new(db: &Arc<Database>) -> Self {
        Self {
            account_service: AccountService::new(db),
            account_repo: AccountRepo::new(db),
        }
    }
}
