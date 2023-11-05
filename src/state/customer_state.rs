use crate::config::database::Database;
use crate::service::customer_service::{CustomerService, CustomerServiceTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct CustomerState {
    pub customer_service: CustomerService,
    pub db: Arc<Database>,
}

impl CustomerState {
    pub fn new(db: &Arc<Database>) -> Self {
        Self {
            customer_service: CustomerService::new(db),
            db: Arc::clone(db),
        }
    }
}
