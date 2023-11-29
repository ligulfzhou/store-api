use crate::config::database::Database;
use crate::service::cates_service::{CateService, CateServiceTrait};
use crate::service::embryo_service::{EmbryoService, EmbryoServiceTrait};
use crate::service::item_service::{ItemService, ItemServiceTrait};
use crate::service::settings_service::{SettingsService, SettingsServiceTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct ExcelState {
    pub item_service: ItemService,
    pub cate_service: CateService,
    pub embryo_service: EmbryoService,
    pub settings_service: SettingsService,
    pub db: Arc<Database>,
}

impl ExcelState {
    pub fn new(db: &Arc<Database>) -> Self {
        Self {
            item_service: ItemService::new(db),
            cate_service: CateService::new(db),
            embryo_service: EmbryoService::new(db),
            settings_service: SettingsService::new(db),
            db: Arc::clone(db),
        }
    }
}
