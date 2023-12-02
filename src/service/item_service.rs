use crate::config::database::{Database, DatabaseTrait};
use crate::constants::DEFAULT_PAGE_SIZE;
use crate::dto::dto_items::{DeleteParams, EditParams, InoutParams, ItemsDto, QueryParams};
use crate::model::embryo::EmbryoModel;
use crate::model::items::{ItemsInOutModel, ItemsModel};
use crate::repository::embryo_repository::{EmbryoRepository, EmbryoRepositoryTrait};
use crate::ERPResult;
use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct ItemService {
    db: Arc<Database>,
    embryo_repo: EmbryoRepository,
}

#[async_trait]
pub trait ItemServiceTrait {
    fn new(db: &Arc<Database>) -> Self;
    async fn get_item_list(&self, params: &QueryParams) -> ERPResult<Vec<ItemsModel>>;
    async fn get_item_count(&self, params: &QueryParams) -> ERPResult<i32>;
    async fn edit_item(&self, params: &EditParams) -> ERPResult<()>;
    async fn delete_item(&self, params: &DeleteParams) -> ERPResult<()>;
    async fn insert_multiple_items(&self, rows: &[ItemsModel]) -> ERPResult<Vec<ItemsModel>>;
    async fn insert_multiple_items_inouts(&self, rows: &[ItemsInOutModel]) -> ERPResult<()>;
    async fn to_items_dto(&self, items: Vec<ItemsModel>) -> ERPResult<Vec<ItemsDto>>;

    async fn add_item_inout(&self, params: &InoutParams, account_id: i32) -> ERPResult<()>;
}

#[async_trait]
impl ItemServiceTrait for ItemService {
    fn new(db: &Arc<Database>) -> Self {
        Self {
            db: Arc::clone(db),
            embryo_repo: EmbryoRepository::new(db),
        }
    }

    async fn get_item_list(&self, params: &QueryParams) -> ERPResult<Vec<ItemsModel>> {
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new("select * from items ");
        if !params.is_empty() {
            sql.push(" where ");

            let mut and = "";

            if !params.name.is_empty() {
                sql.push(&format!("{} name = ", and))
                    .push_bind(&params.name);
                and = " and ";
            }

            if params.cate1_id != 0 {
                sql.push(&format!("{} cate1_id = ", and))
                    .push_bind(params.cate1_id);
                and = " and ";
            }
            if params.cate2_id != 0 {
                sql.push(&format!("{} cate2_id = ", and))
                    .push_bind(params.cate2_id);
                and = " and ";
            }

            if !params.number.is_empty() {
                sql.push(&format!("{} number = ", and))
                    .push_bind(&params.number);
                and = " and ";
            }
            if !params.barcode.is_empty() {
                sql.push(&format!("{} barcode = ", and))
                    .push_bind(&params.barcode);
                and = " and ";
            }

            if !params.create_time_st.is_empty() && !params.create_time_ed.is_empty() {
                sql.push(&format!(" {} create_time >= ", and))
                    .push_bind(&params.create_time_st)
                    .push(&format!(" {} create_time <= ", and))
                    .push_bind(&params.create_time_ed);
            }
        }
        //     let field = param.sorter_field.as_deref().unwrap_or("id");
        //     let order = param.sorter_order.as_deref().unwrap_or("desc");
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
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new("select count(1) from items ");
        if !params.is_empty() {
            sql.push(" where ");

            let mut and = "";
            if !params.name.is_empty() {
                sql.push(&format!("{} name = ", and))
                    .push_bind(&params.name);
                and = " and ";
            }

            if params.cate1_id != 0 {
                sql.push(&format!("{} cate1_id = ", and))
                    .push_bind(params.cate1_id);
                and = " and ";
            }
            if params.cate2_id != 0 {
                sql.push(&format!("{} cate2_id = ", and))
                    .push_bind(params.cate2_id);
                and = " and ";
            }

            if !params.number.is_empty() {
                sql.push(&format!("{} number= ", and))
                    .push_bind(&params.number);
                and = " and ";
            }
            if !params.barcode.is_empty() {
                sql.push(&format!("{} barcode= ", and))
                    .push_bind(&params.barcode);
                and = " and ";
            }

            if !params.create_time_st.is_empty() && !params.create_time_ed.is_empty() {
                sql.push(&format!(" {} create_time >= ", and))
                    .push_bind(&params.create_time_st)
                    .push(&format!(" {} create_time <= ", and))
                    .push_bind(&params.create_time_ed);
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
                    insert into items (images, name, size, color, cate1_id, cate2_id, unit,
                     price, cost, notes, number, barcode)
                    values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);
                    "#,
                    &params.images,
                    params.name,
                    params.size,
                    params.color,
                    params.cate1_id,
                    params.cate2_id,
                    params.unit,
                    &params.price,
                    params.cost,
                    params.notes,
                    params.number,
                    params.barcode,
                )
                .execute(self.db.get_pool())
                .await?;
            }
            _ => {
                // 修改item
                sqlx::query!(
                    r#"
                    update items set images=$1, name=$2, size=$3, color=$4, cate1_id=$5, cate2_id=$6,
                     unit=$7, price=$8, cost=$9, notes=$10, number=$11, barcode=$12
                    where id=$13"#,
                    &params.images,
                    params.name,
                    params.size,
                    params.color,
                    params.cate1_id,
                    params.cate2_id,
                    params.unit,
                    &params.price,
                    params.cost,
                    params.notes,
                    params.number,
                    params.barcode,
                    params.id,
                )
                .execute(self.db.get_pool())
                .await?;
            }
        };

        Ok(())
    }

    async fn delete_item(&self, params: &DeleteParams) -> ERPResult<()> {
        sqlx::query!("delete from items where id = $1", params.id)
            .execute(self.db.get_pool())
            .await?;

        Ok(())
    }

    // todo
    async fn insert_multiple_items(&self, rows: &[ItemsModel]) -> ERPResult<Vec<ItemsModel>> {
        let mut query_builder: QueryBuilder<Postgres> =
                    QueryBuilder::new("insert into items (images, name, size, color, cate1_id, cate2_id, unit, price, cost, notes, number, barcode) ");

        query_builder.push_values(rows, |mut b, item| {
            b.push_bind(item.images.clone())
                .push_bind(item.name.clone())
                .push_bind(item.size.clone())
                .push_bind(item.color.clone())
                .push_bind(item.cate1_id)
                .push_bind(item.cate2_id)
                .push_bind(item.unit.clone())
                .push_bind(item.price)
                .push_bind(item.cost)
                .push_bind(item.notes.clone())
                .push_bind(item.number.clone())
                .push_bind(item.barcode.clone());
        });

        query_builder.push(" returning *;");

        let items = query_builder
            .build_query_as::<ItemsModel>()
            .fetch_all(self.db.get_pool())
            .await?;
        // query_builder.build().execute(self.db.get_pool()).await?;

        Ok(items)
    }

    async fn insert_multiple_items_inouts(&self, rows: &[ItemsInOutModel]) -> ERPResult<()> {
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("insert into item_inout (account_id, item_id, count, in_true_out_false, via, order_id) ");

        query_builder.push_values(rows, |mut b, item| {
            b.push_bind(item.account_id)
                .push_bind(item.item_id)
                .push_bind(item.count)
                .push_bind(item.in_true_out_false)
                .push_bind(item.via.clone())
                .push_bind(item.order_id);
        });

        query_builder.push(" returning id;");

        query_builder.build().execute(self.db.get_pool()).await?;
        Ok(())
    }

    async fn to_items_dto(&self, items: Vec<ItemsModel>) -> ERPResult<Vec<ItemsDto>> {
        let item_ids = items.iter().map(|item| item.id).collect::<Vec<_>>();
        let item_id_to_count = sqlx::query!(
            r#"
            select item_id, sum(count) 
            from item_inout 
            where item_id = any($1)
            group by item_id 
            "#,
            &item_ids
        )
        .fetch_all(self.db.get_pool())
        .await?
        .into_iter()
        .map(|r| (r.item_id, r.sum.unwrap_or(0) as i32))
        .collect::<HashMap<_, _>>();

        let cate_id_to_name = sqlx::query!("select id, name from cates")
            .fetch_all(self.db.get_pool())
            .await?
            .into_iter()
            .map(|item| (item.id, item.name))
            .collect::<HashMap<_, _>>();

        let numbers = items
            .iter()
            .map(|item| item.number.clone())
            .collect::<Vec<_>>();

        let embryos = sqlx::query_as!(
            EmbryoModel,
            "select * from embryos where number=any($1)",
            &numbers
        )
        .fetch_all(self.db.get_pool())
        .await?;

        let embryo_dtos = self.embryo_repo.embryos_to_embryo_dtos(embryos).await?;

        let empty = "".to_string();
        let items_dto = items
            .into_iter()
            .map(|item| {
                let cate1 = cate_id_to_name.get(&item.cate1_id).unwrap_or(&empty);
                let cate2 = cate_id_to_name.get(&item.cate2_id).unwrap_or(&empty);
                let count = item_id_to_count.get(&item.id).unwrap_or(&0);
                ItemsDto::from(item, *count, cate1, cate2, None)
            })
            .collect::<Vec<_>>();

        Ok(items_dto)
    }

    async fn add_item_inout(&self, params: &InoutParams, account_id: i32) -> ERPResult<()> {
        let count = match params.in_out {
            true => params.count,
            _ => -params.count,
        };
        sqlx::query!(
            r#"
            insert into item_inout (account_id, item_id, count, in_true_out_false, via) 
            values ($1, $2, $3, $4, $5);
            "#,
            account_id,
            params.id,
            count,
            params.in_out,
            "form"
        )
        .execute(self.db.get_pool())
        .await?;

        Ok(())
    }
}
