use crate::ERPResult;
use sqlx::{Pool, Postgres, QueryBuilder};

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct ItemsModel {
    pub id: i32,
    pub brand: String,    // 品牌
    pub cates1_id: i32,   // 产品大类
    pub cates2_id: i32,   // 产品小类
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
            QueryBuilder::new("insert into items (cates1_id, cates2_id, goods_no, color, name, size, unit, barcode, sell_price, buy_price ) ");

        query_builder.push_values(rows, |mut b, item| {
            b.push_bind(item.cates1_id)
                .push_bind(item.cates2_id)
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
        // .map_err(ERPError::DBError)?;

        Ok(())
    }
}
