use crate::config::database::{Database, DatabaseTrait};
use crate::model::items::ItemsModel;
use crate::ERPResult;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct ItemService {
    pub db: Arc<Database>,
}

#[async_trait]
pub trait ItemServiceTrait {
    fn new(db: &Arc<Database>) -> Self;

    async fn get_item_list(&self) -> ERPResult<Vec<ItemsModel>>;

    async fn edit_item(&self) -> ERPResult<()>;

    async fn delete_item(&self) -> ERPResult<()>;
}

#[async_trait]
impl ItemServiceTrait for ItemService {
    fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }

    async fn get_item_list(&self) -> ERPResult<Vec<ItemsModel>> {
        todo!()
    }

    async fn edit_item(&self) -> ERPResult<()> {
        todo!()
    }

    async fn delete_item(&self) -> ERPResult<()> {
        todo!()
    }
}
