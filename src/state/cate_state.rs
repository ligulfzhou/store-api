use crate::config::database::Database;
use crate::service::cates_service::{CateService, CateServiceTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct CateState {
    pub cate_service: CateService,
    pub db: Arc<Database>,
}

impl CateState {
    pub fn new(db: &Arc<Database>) -> Self {
        Self {
            cate_service: CateService::new(db),
            db: Arc::clone(db),
        }
    }
}
