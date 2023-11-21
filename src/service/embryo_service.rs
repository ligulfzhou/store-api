use crate::config::database::{Database, DatabaseTrait};
use crate::constants::DEFAULT_PAGE_SIZE;
use crate::dto::dto_embryo::{DeleteParams, EditParams, QueryParams};
use crate::model::embryo::EmbryoModel;
use crate::model::items::ItemsModel;
use crate::ERPResult;
use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct EmbryoService {
    pub db: Arc<Database>,
}

#[async_trait]
pub trait EmbryoServiceTrait {
    fn new(db: &Arc<Database>) -> Self;
    async fn get_item_list(&self, params: &QueryParams) -> ERPResult<Vec<ItemsModel>>;
    async fn get_item_count(&self, params: &QueryParams) -> ERPResult<i32>;
    async fn edit_item(&self, params: &EditParams) -> ERPResult<()>;
    async fn delete_item(&self, params: &DeleteParams) -> ERPResult<()>;
    async fn insert_multiple_items(&self, rows: &[EmbryoModel]) -> ERPResult<Vec<EmbryoModel>>;
}

#[async_trait]
impl EmbryoServiceTrait for EmbryoService {
    fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }
    async fn get_item_list(&self, params: &QueryParams) -> ERPResult<Vec<ItemsModel>> {
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new("select * from embryos ");
        if !params.is_empty() {
            let mut and = "";
            if !params.name.is_empty() {
                sql.push(&format!("{} name= ", and))
                    .push_bind(params.name.deref());
                and = " and ";
            }

            if !params.number.is_empty() {
                sql.push(&format!("{} number= ", and))
                    .push_bind(params.number.deref());
                and = " and ";
            }

            if !params.color.is_empty() {
                sql.push(&format!("{} color= ", and))
                    .push_bind(params.color.deref());
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
            .build_query_as::<ItemsModel>()
            .fetch_all(self.db.get_pool())
            .await?;

        Ok(items)
    }

    async fn get_item_count(&self, params: &QueryParams) -> ERPResult<i32> {
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new("select count(1) from embryos ");
        if !params.is_empty() {
            let mut and = "";
            if !params.name.is_empty() {
                sql.push(&format!("{} name= ", and))
                    .push_bind(params.name.deref());
                and = " and ";
            }

            if !params.number.is_empty() {
                sql.push(&format!("{} number= ", and))
                    .push_bind(params.number.deref());
                and = " and ";
            }

            if !params.color.is_empty() {
                sql.push(&format!("{} color= ", and))
                    .push_bind(params.color.deref());
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

    async fn delete_item(&self, params: &DeleteParams) -> ERPResult<()> {
        sqlx::query!("delete from embryos where id = $1", params.id)
            .execute(self.db.get_pool())
            .await?;

        Ok(())
    }

    // todo
    async fn insert_multiple_items(&self, rows: &[EmbryoModel]) -> ERPResult<Vec<EmbryoModel>> {
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
}
