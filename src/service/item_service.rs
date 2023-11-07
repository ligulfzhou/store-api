use crate::config::database::{Database, DatabaseTrait};
use crate::constants::DEFAULT_PAGE_SIZE;
use crate::dto::dto_items::{DeleteParams, EditParams, QueryParams};
use crate::model::items::ItemsModel;
use crate::ERPResult;
use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct ItemService {
    pub db: Arc<Database>,
}

#[async_trait]
pub trait ItemServiceTrait {
    fn new(db: &Arc<Database>) -> Self;

    async fn get_item_list(&self, params: &QueryParams) -> ERPResult<Vec<ItemsModel>>;
    async fn get_item_count(&self, params: &QueryParams) -> ERPResult<i32>;

    async fn edit_item(&self, params: &EditParams) -> ERPResult<()>;

    async fn delete_item(&self, params: &DeleteParams) -> ERPResult<()>;
}

#[async_trait]
impl ItemServiceTrait for ItemService {
    fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }

    async fn get_item_list(&self, params: &QueryParams) -> ERPResult<Vec<ItemsModel>> {
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new("select * from items ");
        if !params.is_empty() {
            let mut and = "";
            if !params.brand.is_empty() {
                sql.push(&format!("{} brand= ", and))
                    .push_bind(params.brand.deref());
                and = " and ";
            }

            if !params.cates1.is_empty() {
                sql.push(&format!("{} cates1= ", and))
                    .push_bind(params.cates1.deref());
                and = " and ";
            }

            if !params.cates2.is_empty() {
                sql.push(&format!("{} cates2= ", and))
                    .push_bind(params.cates2.deref());
                and = " and ";
            }
            if !params.goods_no.is_empty() {
                sql.push(&format!("{} goods_no= ", and))
                    .push_bind(params.goods_no.deref());
                and = " and ";
            }
            if !params.name.is_empty() {
                sql.push(&format!("{} name= ", and))
                    .push_bind(params.name.deref());
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
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new("select count(1) from items ");
        if !params.is_empty() {
            let mut and = "";
            if !params.brand.is_empty() {
                sql.push(&format!("{} brand= ", and))
                    .push_bind(params.brand.deref());
                and = " and ";
            }

            if !params.cates1.is_empty() {
                sql.push(&format!("{} cates1= ", and))
                    .push_bind(params.cates1.deref());
                and = " and ";
            }

            if !params.cates2.is_empty() {
                sql.push(&format!("{} cates2= ", and))
                    .push_bind(params.cates2.deref());
                and = " and ";
            }
            if !params.goods_no.is_empty() {
                sql.push(&format!("{} goods_no= ", and))
                    .push_bind(params.goods_no.deref());
                and = " and ";
            }
            if !params.name.is_empty() {
                sql.push(&format!("{} name= ", and))
                    .push_bind(params.name.deref());
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
                    insert into items (brand, cates1, cates2, goods_no, color, name, size, unit,
                     barcode, sell_price, buy_price, images, supplier, material, pcs, weight, 
                     english_name, description, notes )
                    values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, 
                     $17, $18, $19);
                    "#,
                    params.brand,
                    params.cates1,
                    params.cates2,
                    params.goods_no,
                    params.color,
                    params.name,
                    params.size,
                    params.unit,
                    params.barcode,
                    params.sell_price,
                    params.buy_price,
                    &params.images,
                    params.supplier,
                    params.material,
                    params.pcs,
                    params.weight,
                    params.english_name,
                    params.description,
                    params.notes,
                )
                .execute(self.db.get_pool())
                .await?;
            }
            _ => {
                // 修改item
                sqlx::query!(
                    r#"
                    update items set brand=$1, cates1=$2, cates2=$3, goods_no=$4, color=$5, name=$6,
                     size=$7, unit=$8, barcode=$9, sell_price=$10, buy_price=$11, images=$12, 
                     supplier=$13, material=$14, pcs=$15, weight=$16, english_name=$17, description=$18, 
                     notes=$19
                    where id=$20"#,
                    params.brand,
                    params.cates1,
                    params.cates2,
                    params.goods_no,
                    params.color,
                    params.name,
                    params.size,
                    params.unit,
                    params.barcode,
                    params.sell_price,
                    params.buy_price,
                    &params.images,
                    params.supplier,
                    params.material,
                    params.pcs,
                    params.weight,
                    params.english_name,
                    params.description,
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
        todo!()
    }
}
