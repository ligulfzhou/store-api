use crate::common::hashmap::key_of_max_value;
use crate::common::string::common_prefix;
use crate::{ERPError, ERPResult};
use chrono::NaiveDate;
use sqlx::{Pool, Postgres, QueryBuilder};
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone, sqlx::FromRow)]
pub struct OrderModel {
    pub id: i32,
    pub customer_no: String,
    pub order_no: String,
    pub order_date: NaiveDate,
    pub delivery_date: Option<NaiveDate>,
    pub is_urgent: bool,       //紧急 ‼️
    pub is_return_order: bool, // 返单
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderGoodsModel {
    pub id: i32,
    pub index: i32,
    pub order_id: i32,
    pub goods_id: i32,
}

impl OrderGoodsModel {
    pub async fn add_rows(
        db: &Pool<Postgres>,
        rows: &[OrderGoodsModel],
    ) -> ERPResult<Vec<OrderGoodsModel>> {
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("insert into order_goods (index, order_id, goods_id) ");

        query_builder.push_values(rows, |mut b, item| {
            b.push_bind(item.index)
                .push_bind(item.order_id)
                .push_bind(item.goods_id);
        });
        query_builder.push(" returning *;");

        let res = query_builder
            .build_query_as::<OrderGoodsModel>()
            .fetch_all(db)
            .await
            .map_err(ERPError::DBError)?;

        Ok(res)
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderItemModel {
    pub id: i32,
    pub order_goods_id: i32, // todo: 应该是比存goods_id好
    pub order_id: i32,
    pub sku_id: i32,
    pub count: i32,
    pub unit: Option<String>,
    // pub purchase_price: Option<i32>,
    pub unit_price: Option<i32>,
    pub total_price: Option<i32>,
    pub notes: String,
}

impl OrderItemModel {
    pub async fn save_to_order_item_table(
        db: &Pool<Postgres>,
        items: &[OrderItemModel],
    ) -> ERPResult<Vec<OrderItemModel>> {
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("insert into order_items (order_goods_id, order_id, sku_id, count, unit, unit_price, total_price) ");

        query_builder.push_values(items, |mut b, item| {
            b.push_bind(item.order_goods_id)
                .push_bind(item.order_id)
                .push_bind(item.sku_id)
                .push_bind(item.count)
                .push_bind(item.unit.as_deref().unwrap_or(""))
                .push_bind(item.unit_price.unwrap_or(0))
                .push_bind(item.total_price.unwrap_or(0));
        });
        query_builder.push(" returning *;");

        let res = query_builder
            .build_query_as::<OrderItemModel>()
            .fetch_all(db)
            .await
            .map_err(ERPError::DBError)?;

        Ok(res)
    }
}

#[derive(Debug, Clone)]
pub struct ExcelOrderGoods {
    pub index: i32,
    pub goods_no: String,
    pub image: Option<String>,
    pub image_des: Option<String>,
    pub name: String,
    pub plating: String,
    pub package_card: Option<String>,
    pub package_card_des: Option<String>,
}

// #[derive(Debug, FromRow)]
// struct GoodsNoId {
//     pub goods_no: String,
//     pub id: i32,
// }
// .build_query_as::<GoodsNoId>()

impl ExcelOrderGoods {
    pub async fn insert_into_goods_table(
        db: &Pool<Postgres>,
        items: &[ExcelOrderGoods],
        customer_no: &str,
    ) -> ERPResult<HashMap<String, i32>> {
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("insert into goods (customer_no, goods_no, image, image_des, name, plating, package_card, package_card_des) ");

        query_builder.push_values(items, |mut b, item| {
            b.push_bind(customer_no)
                .push_bind(item.goods_no.clone())
                .push_bind(item.image.as_deref().unwrap_or(""))
                .push_bind(item.image_des.as_deref().unwrap_or(""))
                .push_bind(item.name.clone())
                .push_bind(item.plating.clone())
                .push_bind(item.package_card.as_deref().unwrap_or(""))
                .push_bind(item.package_card_des.as_deref().unwrap_or(""));
        });
        query_builder.push(" returning goods_no, id;");

        let res = query_builder
            .build_query_as::<(String, i32)>()
            .fetch_all(db)
            .await
            .map_err(ERPError::DBError)?
            .into_iter()
            .map(|item| (item.0, item.1))
            .collect::<HashMap<String, i32>>();

        Ok(res)
    }
}

#[derive(Debug, Clone)]
pub struct ExcelOrderItems {
    /// 颜色
    pub color: String,
    /// 颜色
    pub color_2: Option<String>,
    /// 尺寸
    pub size: Option<String>,
    /// 条码
    pub barcode: Option<String>,
    /// 数量
    pub count: i32,
    /// 进货价
    pub purchase_price: Option<i32>,
    /// 单位
    pub unit: Option<String>,
    /// 单价
    pub unit_price: Option<i32>,
    /// 金额
    pub total_price: Option<i32>,
    /// 备注
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ExcelOrderGoodsWithItems {
    pub goods: ExcelOrderGoods,
    pub items: Vec<OrderItemExcel>,
}

#[derive(Debug, Clone)]
pub struct ExcelOrderV2 {
    pub info: OrderInfo,
    pub items: Vec<ExcelOrderGoodsWithItems>,
    pub exists: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ExcelOrder {
    pub info: OrderInfo,
    pub items: Vec<OrderItemExcel>,
    pub exists: bool,
}

#[derive(Default, Debug, Clone)]
pub struct OrderInfo {
    pub customer_no: String,
    pub order_no: String,
    pub order_date: NaiveDate,
    pub delivery_date: Option<NaiveDate>,
    pub is_return_order: bool,
    pub is_urgent: bool,
}

#[derive(Debug, Default, Clone)]
pub struct OrderItemExcel {
    pub index: i32,
    pub package_card: Option<String>,
    pub package_card_des: Option<String>,
    /// 商品唯一编号
    pub goods_no: String,
    // /// 商品编号
    // pub goods_no_2: Option<String>, // 反正用处并不大
    /// sku编号 //只有L1005有这个字段
    pub sku_no: Option<String>,
    /// 商品图片
    pub image: Option<String>,
    /// 商品的图片描述
    pub image_des: Option<String>,
    /// 商品名
    pub name: String,
    /// 电镀
    pub plating: String,
    /// 色号/颜色
    pub color: String,
    /// 颜色
    pub color_2: Option<String>,
    /// 尺寸
    pub size: Option<String>,
    /// 条码
    pub barcode: Option<String>,
    /// 数量
    pub count: i32,
    /// 进货价
    pub purchase_price: Option<i32>,
    /// 单位
    pub unit: Option<String>,
    /// 单价
    pub unit_price: Option<i32>,
    /// 金额
    pub total_price: Option<i32>,
    /// 备注
    pub notes: Option<String>,
}

impl OrderItemExcel {
    pub fn pick_up_package(items: &Vec<OrderItemExcel>) -> (String, String) {
        let mut package_card: Option<String> = None;
        let mut package_card_des: Option<String> = None;

        for item in items {
            if package_card.is_none() && item.package_card.is_some() {
                package_card = item.package_card.clone();
            }
            if package_card_des.is_none() && item.package_card_des.is_some() {
                package_card_des = item.package_card_des.clone();
            }
        }

        (
            package_card.unwrap_or("".to_string()),
            package_card_des.unwrap_or("".to_string()),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderItemMaterialModel {
    pub id: i32,
    pub order_id: i32,
    pub order_item_id: i32,
    pub name: String,
    pub color: String,
    // material_id   integer, -- 材料ID  (暂时先不用)
    pub single: Option<i32>,   //  integer, -- 单数      ？小数
    pub count: i32,            //  integer, -- 数量      ？小数
    pub total: Option<i32>,    //  integer, -- 总数(米)  ? 小数
    pub stock: Option<i32>,    //  integer, -- 库存 ?
    pub debt: Option<i32>,     //  integer, -- 欠数
    pub notes: Option<String>, //  text,     -- 备注
}
