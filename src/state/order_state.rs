use crate::config::database::Database;
use crate::service::customer_service::{CustomerService, CustomerServiceTrait};
use crate::service::item_service::{ItemService, ItemServiceTrait};
use crate::service::settings_service::{SettingsService, SettingsServiceTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct OrderState {
    pub customer_service: CustomerService,
    pub settings_service: SettingsService,
    pub item_service: ItemService,
    pub db: Arc<Database>,
}

impl OrderState {
    pub fn new(db: &Arc<Database>) -> Self {
        Self {
            customer_service: CustomerService::new(db),
            settings_service: SettingsService::new(db),
            item_service: ItemService::new(db),
            db: Arc::clone(db),
        }
    }
}
