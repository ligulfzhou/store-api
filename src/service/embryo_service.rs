use crate::config::database::{Database, DatabaseTrait};
use crate::constants::DEFAULT_PAGE_SIZE;
use crate::dto::dto_embryo::{EditParams, EmbryoDto, InoutParams, QueryParams};
use crate::dto::dto_excel::EmbryoExcelDto;
use crate::dto::GenericDeleteParams;
use crate::model::embryo::{EmbryoInOutBucketModal, EmbryoInOutModel, EmbryoModel};
use crate::repository::embryo_repository::{EmbryoRepository, EmbryoRepositoryTrait};
use crate::{ERPError, ERPResult};
use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct EmbryoService {
    db: Arc<Database>,
    pub embryo_repo: EmbryoRepository,
}

#[async_trait]
pub trait EmbryoServiceTrait {
    fn new(db: &Arc<Database>) -> Self;
    async fn get_item_list(&self, params: &QueryParams) -> ERPResult<Vec<EmbryoModel>>;
    async fn get_item_count(&self, params: &QueryParams) -> ERPResult<i32>;
    async fn edit_item(&self, params: &EditParams) -> ERPResult<()>;
    async fn delete_item(&self, params: &GenericDeleteParams) -> ERPResult<()>;
    async fn insert_multiple_items(&self, rows: &[EmbryoExcelDto]) -> ERPResult<Vec<EmbryoModel>>;
    async fn insert_multiple_items_inouts(&self, rows: &[EmbryoInOutModel]) -> ERPResult<()>;
    async fn add_item_inout(&self, params: &InoutParams, account_id: i32) -> ERPResult<()>;
    async fn embryos_to_embryo_dtos(&self, embryos: Vec<EmbryoModel>) -> ERPResult<Vec<EmbryoDto>>;
    // async fn get_embryo_dtos_with_numbers(&self, numbers: &[String]) -> ERPResult<Vec<EmbryoDto>>;
    async fn add_inout_bucket(
        &self,
        bucket: EmbryoInOutBucketModal,
    ) -> ERPResult<EmbryoInOutBucketModal>;
}

#[async_trait]
impl EmbryoServiceTrait for EmbryoService {
    fn new(db: &Arc<Database>) -> Self {
        Self {
            db: Arc::clone(db),
            embryo_repo: EmbryoRepository::new(db),
        }
    }
    async fn get_item_list(&self, params: &QueryParams) -> ERPResult<Vec<EmbryoModel>> {
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new("select * from embryos ");

        if !params.is_empty() {
            sql.push(" where ");
            let mut and = "";
            if !params.name.is_empty() {
                sql.push(&format!("{} name = ", and))
                    .push_bind(&params.name);
                and = " and ";
            }

            if !params.number.is_empty() {
                sql.push(&format!("{} number = ", and))
                    .push_bind(&params.number);
                and = " and ";
            }
        }
        //     let field = param.sorter_field.as_deref().unwrap_or("id");
        //     let order = param.sorter_order.as_deref().unwrap_or("desc");
        //
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        let offset = (page - 1) * page_size;

        sql.push(format!(
            " order by id desc limit {} offset {}",
            page_size, offset
        ));

        let items = sql
            .build_query_as::<EmbryoModel>()
            .fetch_all(self.db.get_pool())
            .await?;

        Ok(items)
    }

    async fn get_item_count(&self, params: &QueryParams) -> ERPResult<i32> {
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new("select count(1) from embryos ");
        if !params.is_empty() {
            sql.push(" where ");
            let mut and = "";
            if !params.name.is_empty() {
                sql.push(&format!("{} name = ", and))
                    .push_bind(&params.name);
                and = " and ";
            }

            if !params.number.is_empty() {
                sql.push(&format!("{} number = ", and))
                    .push_bind(&params.number);
                and = " and ";
            }
        }

        let count = sql
            .build_query_as::<(i64,)>()
            .fetch_one(self.db.get_pool())
            .await?
            .0 as i32;

        Ok(count)
    }

    async fn edit_item(&self, params: &EditParams) -> ERPResult<()> {
        match params.id {
            0 => {
                // 新增item
                sqlx::query!(
                    r#"
                    insert into embryos (images, name, color, unit, number, notes)
                    values ($1, $2, $3, $4, $5, $6);
                    "#,
                    &params.images,
                    params.name,
                    params.color,
                    params.unit,
                    params.number,
                    params.notes,
                )
                .execute(self.db.get_pool())
                .await?;
            }
            _ => {
                // 修改item
                sqlx::query!(
                    r#"
                    update embryos set images=$1, name=$2, color=$3, unit=$4, number=$5, notes=$6
                    where id=$7
                    "#,
                    &params.images,
                    params.name,
                    params.color,
                    params.unit,
                    params.number,
                    params.notes,
                    params.id,
                )
                .execute(self.db.get_pool())
                .await?;
            }
        };

        Ok(())
    }

    async fn delete_item(&self, params: &GenericDeleteParams) -> ERPResult<()> {
        sqlx::query!("delete from embryos where id = $1", params.id)
            .execute(self.db.get_pool())
            .await?;

        Ok(())
    }

    async fn insert_multiple_items(&self, rows: &[EmbryoExcelDto]) -> ERPResult<Vec<EmbryoModel>> {
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("insert into embryos (images, name,  color, unit, number, notes)");

        query_builder.push_values(rows, |mut b, item| {
            b.push_bind(item.images.clone())
                .push_bind(item.name.clone())
                .push_bind(item.color.clone())
                .push_bind(item.unit.clone())
                .push_bind(item.number.clone())
                .push_bind(item.notes.clone());
        });

        query_builder.push(" returning *;");

        let embryos = query_builder
            .build_query_as::<EmbryoModel>()
            .fetch_all(self.db.get_pool())
            .await?;

        Ok(embryos)
    }

    async fn insert_multiple_items_inouts(&self, rows: &[EmbryoInOutModel]) -> ERPResult<()> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "insert into embryo_inout (bucket_id, embryo_id, count, current_price, current_total) ",
        );

        query_builder.push_values(rows, |mut b, item| {
            b.push_bind(item.bucket_id)
                .push_bind(item.embryo_id)
                .push_bind(item.count)
                .push_bind(item.current_price)
                .push_bind(item.current_total);
        });

        query_builder.push(" returning id;");

        query_builder.build().execute(self.db.get_pool()).await?;
        Ok(())
    }

    async fn add_item_inout(&self, params: &InoutParams, account_id: i32) -> ERPResult<()> {
        let embryo = sqlx::query_as!(
            EmbryoModel,
            "select * from embryos where id = $1",
            params.id
        )
        .fetch_one(self.db.get_pool())
        .await
        .map_err(|_err| ERPError::NotFound("库存胚未找到".to_string()))?;

        let count = match params.in_out {
            true => params.count,
            _ => -params.count,
        };

        let bucket_id = sqlx::query!(
            r#"
            insert into embryo_inout_bucket (account_id, in_true_out_false, via)
            values ($1, $2, $3) 
            returning id
            "#,
            account_id,
            params.in_out,
            "form"
        )
        .fetch_one(self.db.get_pool())
        .await?
        .id;

        sqlx::query!(
            r#"
            insert into embryo_inout (bucket_id, embryo_id, count, current_price, current_total) 
            values ($1, $2, $3, $4, $5);
            "#,
            bucket_id,
            params.id,
            count,
            embryo.cost,
            embryo.cost * count
        )
        .execute(self.db.get_pool())
        .await?;

        Ok(())
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

    async fn add_inout_bucket(
        &self,
        bucket: EmbryoInOutBucketModal,
    ) -> ERPResult<EmbryoInOutBucketModal> {
        let bucket = sqlx::query_as!(
            EmbryoInOutBucketModal,
            r#"
            insert into embryo_inout_bucket (account_id, in_true_out_false, via)
            values ($1, $2, $3)
            returning *
            "#,
            bucket.account_id,
            bucket.in_true_out_false,
            bucket.via,
        )
        .fetch_one(self.db.get_pool())
        .await?;

        Ok(bucket)
    }
}
