use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct EmbryoModel {
    pub id: i32,
    pub images: Vec<String>,
    pub name: String,
    pub color: String,
    pub unit: String,
    pub notes: String,
    pub number: String,
    pub create_time: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct EmbryoInOutModel {
    pub id: i32,
    pub account_id: i32,            // 商品图片
    pub embryo_id: i32,             // 产品名称
    pub count: i32,                 // 数量
    pub in_true_out_false: bool,    // 增加还是减少
    pub via: String,                // 规格
    pub create_time: NaiveDateTime, // 创建时间
}
