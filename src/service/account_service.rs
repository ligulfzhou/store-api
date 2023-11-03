use crate::config::database::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct AccountService {
    pub db: Arc<Database>,
}

pub trait AccountServiceTrait {
    fn new(db: &Arc<Database>) -> Self;
}

impl AccountServiceTrait for AccountService {
    fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }
}
