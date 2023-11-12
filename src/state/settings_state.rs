use crate::config::database::Database;
use crate::service::settings_service::{SettingsService, SettingsServiceTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct SettingsState {
    pub settings_service: SettingsService,
    pub db: Arc<Database>,
}

impl SettingsState {
    pub fn new(db: &Arc<Database>) -> Self {
        Self {
            settings_service: SettingsService::new(db),
            db: Arc::clone(db),
        }
    }
}
