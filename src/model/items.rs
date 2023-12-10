use chrono::NaiveDateTime;
use sqlx::FromRow;

/* 图片（可多张）, 名称, 规格, 颜色, 类别（大类+小类）,单位, 售价, 成本, 备注（可空）, 编号（6位数字，688001，688002...)*/
#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct ItemsModel {
    pub id: i32,
    pub images: Vec<String>,        // 商品图片
    pub name: String,               // 产品名称
    pub size: String,               // 规格
    pub color: String,              // 颜色
    pub cate1_id: i32,              // 大类ID
    pub cate2_id: i32,              // 小类ID
    pub unit: String,               // 单位
    pub price: i32,                 // 标准售价
    pub cost: i32,                  // 成本
    pub notes: String,              // 备注
    pub number: String,             // 货号
    pub barcode: String,            // 条码
    pub create_time: NaiveDateTime, // 创建时间
}

#[derive(Debug, Serialize, Clone, FromRow)]
pub struct ItemInOutBucketModal {
    pub id: i32,
    pub account_id: i32,            // 商品图片
    pub in_true_out_false: bool,    // 增加还是减少
    pub via: String,                // 规格
    pub order_id: i32,              // 颜色
    pub create_time: NaiveDateTime, // 创建时间
}

#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct ItemsInOutModel {
    pub id: i32,
    pub bucket_id: i32,
    pub item_id: i32,
    pub count: i32,
    pub current_cost: i32,
    pub current_total: i32,
}
