use crate::config::database::Database;
use crate::service::cates_service::{CateService, CateServiceTrait};
use crate::service::item_service::{ItemService, ItemServiceTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct ItemState {
    pub item_service: ItemService,
    pub cate_service: CateService,
    pub db: Arc<Database>,
}

impl ItemState {
    pub fn new(db: &Arc<Database>) -> Self {
        Self {
            item_service: ItemService::new(db),
            cate_service: CateService::new(db),
            db: Arc::clone(db),
        }
    }
}
