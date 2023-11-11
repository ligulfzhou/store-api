use crate::config::database::{Database, DatabaseTrait};
use crate::dto::dto_cates::CateDto;
use crate::model::cates::CateModel;
use crate::ERPResult;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct CateService {
    pub db: Arc<Database>,
}

#[async_trait]
pub trait CateServiceTrait {
    fn new(db: &Arc<Database>) -> Self;
    async fn get_all_cates(&self) -> ERPResult<Vec<CateDto>>;

    async fn update_cates(&self) -> ERPResult<()>;

    async fn extract_cates(&self) -> ERPResult<()>;
}

#[async_trait]
impl CateServiceTrait for CateService {
    fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }

    async fn get_all_cates(&self) -> ERPResult<Vec<CateDto>> {
        // let cates_dto = sqlx::query_as!(CateModel, "select * from cates order by index, id;")
        //     .fetch_all(self.db.get_pool())
        //     .await?
        //     .into_iter()
        //     .map(CateDto::from)
        //     .collect::<Vec<CateDto>>();

        // Ok(cates_dto)
        Ok(vec![])
    }

    async fn update_cates(&self) -> ERPResult<()> {
        todo!()
    }

    async fn extract_cates(&self) -> ERPResult<()> {
        todo!()
    }
}
