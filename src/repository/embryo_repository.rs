use crate::config::database::{Database, DatabaseTrait};
use crate::dto::dto_embryo::{EmbryoDto, EmbryoInOutDto};
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
    async fn get_embryo(&self, embryo_id: i32) -> ERPResult<EmbryoModel>;

    async fn inout_list_of_embryo(
        &self,
        embryo_id: i32,
        _account: &str,
        page: i32,
        page_size: i32,
    ) -> ERPResult<Vec<EmbryoInOutDto>>;

    async fn inout_list_of_embryo_count(&self, embryo_id: i32) -> ERPResult<i32>;
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

    async fn get_embryo(&self, embryo_id: i32) -> ERPResult<EmbryoModel> {
        Ok(sqlx::query_as!(
            EmbryoModel,
            "select * from embryos where id = $1",
            embryo_id
        )
        .fetch_one(self.db.get_pool())
        .await?)
    }

    async fn inout_list_of_embryo(
        &self,
        embryo_id: i32,
        _account: &str,
        page: i32,
        page_size: i32,
    ) -> ERPResult<Vec<EmbryoInOutDto>> {
        let embryo = self.get_embryo(embryo_id).await?;

        let offset = (page - 1) * page_size;
        let inouts = sqlx::query_as!(
            EmbryoInOutDto,
            r#"
            select 
                ei.*, 
                eib.in_true_out_false, eib.via, eib.create_time, eib.account_id,
                e.name as embryo_name, e.unit, e.number,
                a.name as account
            from 
                embryo_inout ei, accounts a, embryo_inout_bucket eib, embryos e
            where ei.bucket_id = eib.id and eib.account_id = a.id and ei.embryo_id=e.id 
                and ei.embryo_id = $1 order by id desc offset $2 limit $3
            "#,
            embryo_id,
            offset as i64,
            page_size as i64
        )
        .fetch_all(self.db.get_pool())
        .await?;

        Ok(inouts)
    }

    async fn inout_list_of_embryo_count(&self, embryo_id: i32) -> ERPResult<i32> {
        Ok(sqlx::query!(
            "select count(1) from embryo_inout where embryo_id = $1",
            embryo_id,
        )
        .fetch_one(self.db.get_pool())
        .await?
        .count
        .unwrap_or(0) as i32)
    }
}
