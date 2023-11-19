use chrono::NaiveDateTime;

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct CateModel {
    pub id: i32,        // SERIAL
    pub index: i32,     // 顺序
    pub name: String,   // 类名
    pub cate_type: i32, // 大类小类， 0 大类， 1小类，再变大，则更小
    pub parent_id: i32, // 父类ID
    pub create_time: NaiveDateTime,
}
