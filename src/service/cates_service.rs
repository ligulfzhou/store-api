use crate::config::database::{Database, DatabaseTrait};
use crate::dto::dto_cates::{CateDto, EditParams};
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

    async fn edit_cates(&self, params: &EditParams) -> ERPResult<()>;

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
    async fn extract_cates(&self) -> ERPResult<()> {
        todo!()
    }

    async fn edit_cates(&self, params: &EditParams) -> ERPResult<()> {
        match params.id {
            0 => {
                // 新增item
                sqlx::query!(
                    r#"
                    insert into cates (index, name, cate_type, parent_id)
                    values ($1, $2, $3, $4);
                    "#,
                    params.index,
                    params.name,
                    params.cate_type,
                    params.parent_id,
                )
                .execute(self.db.get_pool())
                .await?;
            }
            _ => {
                // 修改item
                sqlx::query!(
                    r#"
                    update cates set index=$1, name=$2, cate_type=$3, parent_id=$4
                    where id=$5
                    "#,
                    params.index,
                    params.name,
                    params.cate_type,
                    params.parent_id,
                    params.id,
                )
                .execute(self.db.get_pool())
                .await?;
            }
        };

        Ok(())
    }
}
