use crate::handler::routes_items::ItemListParam;
use crate::{ERPError, ERPResult};
use sqlx::{Pool, Postgres, QueryBuilder};

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct ItemsModel {
    pub id: i32,
    pub brand: String,    // 品牌
    pub cates1: String,   // 产品大类
    pub cates2: String,   // 产品小类
    pub goods_no: String, // 货号
    pub color: String,    // 颜色
    pub name: String,     // 产品名称
    pub size: String,     // 规格
    pub unit: String,     // 单位
    pub barcode: String,  // 条码
    pub sell_price: i32,  // 标准售价
    pub buy_price: i32,   // 进货价
}

impl ItemsModel {
    pub async fn insert_multiple_items(db: &Pool<Postgres>, rows: &[ItemsModel]) -> ERPResult<()> {
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("insert into items (cates1, cates2, goods_no, color, name, size, unit, barcode, sell_price, buy_price ) ");

        query_builder.push_values(rows, |mut b, item| {
            b.push_bind(item.cates1.clone())
                .push_bind(item.cates2.clone())
                .push_bind(item.goods_no.clone())
                .push_bind(item.color.clone())
                .push_bind(item.name.clone())
                .push_bind(item.size.clone())
                .push_bind(item.unit.clone())
                .push_bind(item.barcode.clone())
                .push_bind(item.sell_price)
                .push_bind(item.buy_price);
        });

        query_builder.push(" returning id;");

        query_builder.build().execute(db).await?;

        Ok(())
    }
}

// for routers
impl ItemsModel {
    pub async fn get_list(
        db: &Pool<Postgres>,
        param: &ItemListParam,
    ) -> ERPResult<Vec<ItemsModel>> {
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new("select * from items ");
        if param.cates1_id.unwrap_or(0) > 0
            || param.cates2_id.unwrap_or(0) > 0
            || param.has_storage.unwrap_or(0) > 0
        {
            sql.push(" where ");
            let mut should_add_and = false;

            if param.cates1_id.unwrap_or(0) > 0 {
                if should_add_and {
                    sql.push(" and cates1_id = ");
                } else {
                    sql.push(" cates1_id = ");
                }

                sql.push_bind(param.cates1_id.unwrap_or(0));
                should_add_and = true;
            }

            if param.cates2_id.unwrap_or(0) > 0 {
                if should_add_and {
                    sql.push(" and cates2_id = ");
                } else {
                    sql.push(" cates2_id = ");
                }
                sql.push_bind(param.cates2_id.unwrap_or(0));
            }
        }

        let field = param.sorter_field.as_deref().unwrap_or("id");
        let order = param.sorter_order.as_deref().unwrap_or("desc");

        sql.push(format!(" order by {} {}", field, order));

        let items = sql
            .build_query_as::<ItemsModel>()
            .fetch_all(db)
            .await
            .map_err(ERPError::DBError)?;

        Ok(items)
    }

    pub async fn get_count(db: &Pool<Postgres>, param: &ItemListParam) -> ERPResult<i32> {
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new("select count(1) from items ");
        if param.cates1_id.unwrap_or(0) > 0
            || param.cates2_id.unwrap_or(0) > 0
            || param.has_storage.unwrap_or(0) > 0
        {
            sql.push(" where ");
            let mut should_add_and = false;

            if param.cates1_id.unwrap_or(0) > 0 {
                if should_add_and {
                    sql.push(" and cates1_id = ");
                } else {
                    sql.push(" cates1_id = ");
                }

                sql.push_bind(param.cates1_id.unwrap_or(0));
                should_add_and = true;
            }

            if param.cates2_id.unwrap_or(0) > 0 {
                if should_add_and {
                    sql.push(" and cates2_id = ");
                } else {
                    sql.push(" cates2_id = ");
                }
                sql.push_bind(param.cates2_id.unwrap_or(0));
            }
        }

        let count = sql
            .build_query_as::<(i64,)>()
            .fetch_one(db)
            .await
            .map_err(ERPError::DBError)?;

        Ok(count.0 as i32)
    }
}
