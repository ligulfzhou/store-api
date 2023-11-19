use crate::config::database::Database;
use crate::service::embryo_service::{EmbryoService, EmbryoServiceTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct EmbryoState {
    pub embryo_service: EmbryoService,
    pub db: Arc<Database>,
}

impl EmbryoState {
    pub fn new(db: &Arc<Database>) -> Self {
        Self {
            embryo_service: EmbryoService::new(db),
            db: Arc::clone(db),
        }
    }
}
