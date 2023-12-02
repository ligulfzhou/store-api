use crate::config::database::{Database, DatabaseTrait};
use crate::dto::dto_embryo::EmbryoDto;
use crate::model::embryo::EmbryoModel;
use crate::ERPResult;
use async_trait::async_trait;
use sqlx;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct EmbryoRepository {
    pub(crate) db: Arc<Database>,
}

#[async_trait]
pub trait EmbryoRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;
    async fn embryos_to_embryo_dtos(&self, embryos: Vec<EmbryoModel>) -> ERPResult<Vec<EmbryoDto>>;
    async fn get_embryo_dtos_with_numbers(&self, numbers: &[String]) -> ERPResult<Vec<EmbryoDto>>;
}

#[async_trait]
impl EmbryoRepositoryTrait for EmbryoRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db: Arc::clone(db_conn),
        }
    }

    async fn embryos_to_embryo_dtos(&self, embryos: Vec<EmbryoModel>) -> ERPResult<Vec<EmbryoDto>> {
        let embryo_ids = embryos.iter().map(|item| item.id).collect::<Vec<_>>();

        let embryo_id_to_count = sqlx::query!(
            r#"
            select embryo_id, sum(count) 
            from embryo_inout 
            where embryo_id = any($1)
            group by embryo_id
            "#,
            &embryo_ids
        )
        .fetch_all(self.db.get_pool())
        .await?
        .into_iter()
        .map(|r| (r.embryo_id, r.sum.unwrap_or(0) as i32))
        .collect::<HashMap<_, _>>();

        let embryo_dtos = embryos
            .into_iter()
            .map(|item| {
                let count = embryo_id_to_count.get(&item.id).unwrap_or(&0);

                EmbryoDto::from(item, *count)
            })
            .collect::<Vec<_>>();

        Ok(embryo_dtos)
    }

    async fn get_embryo_dtos_with_numbers(&self, numbers: &[String]) -> ERPResult<Vec<EmbryoDto>> {
        let embryos = sqlx::query_as!(
            EmbryoModel,
            "select * from embryos where number=any($1)",
            numbers
        )
        .fetch_all(self.db.get_pool())
        .await?;

        let embryo_dtos = self.embryos_to_embryo_dtos(embryos).await?;

        Ok(embryo_dtos)
    }
}
