use crate::{ERPError, ERPResult};
use sqlx::{Pool, Postgres, QueryBuilder};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct CateModel {
    pub id: i32,             // SERIAL
    pub index: i32,          // 顺序
    pub name: String,        // 类名
    pub cate_type: i32,      // 大类小类， 0 大类， 1小类，再变大，则更小
    pub parent_name: String, // 父类
}

impl CateModel {
    pub async fn insert_multiple_cate1(
        db: &Pool<Postgres>,
        rows: &[String],
    ) -> ERPResult<HashMap<String, i32>> {
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("insert into cates (name, cate_type, parent_name) ");

        query_builder.push_values(rows, |mut b, item| {
            b.push_bind(item).push_bind(0).push_bind(0);
        });
        query_builder.push(" returning *;");

        let res = query_builder
            .build_query_as::<CateModel>()
            .fetch_all(db)
            .await
            .map_err(ERPError::DBError)?
            .into_iter()
            .map(|cate| (cate.name, cate.id))
            .collect();

        Ok(res)
    }

    pub async fn insert_multiple_cate2(
        db: &Pool<Postgres>,
        rows: Vec<CateModel>,
    ) -> ERPResult<Vec<CateModel>> {
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("insert into cates (name, cate_type, parent_name) ");

        query_builder.push_values(rows, |mut b, item| {
            b.push_bind(item.name)
                .push_bind(item.cate_type)
                .push_bind(item.parent_name);
        });
        query_builder.push(" returning *;");

        let res = query_builder
            .build_query_as::<CateModel>()
            .fetch_all(db)
            .await
            .map_err(ERPError::DBError)?;

        Ok(res)
    }
}
