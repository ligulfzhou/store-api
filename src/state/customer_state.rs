use crate::config::database::Database;
use crate::service::customer_service::{CustomerService, CustomerServiceTrait};
use crate::service::settings_service::{SettingsService, SettingsServiceTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct CustomerState {
    pub customer_service: CustomerService,
    pub settings_service: SettingsService,
    pub db: Arc<Database>,
}

impl CustomerState {
    pub fn new(db: &Arc<Database>) -> Self {
        Self {
            customer_service: CustomerService::new(db),
            settings_service: SettingsService::new(db),
            db: Arc::clone(db),
        }
    }
}
