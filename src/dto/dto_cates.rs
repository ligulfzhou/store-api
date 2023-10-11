/*
    pub id: i32,        // SERIAL
    pub name: String,   // 类名
    pub cate_type: i32, // 大类小类， 0 大类， 1小类，再变大，则更小
    pub parent_id: i32, // 父类
*/

#[derive(Debug, Deserialize, Serialize)]
pub struct CatesDto {
    pub id: i32,
    pub customer_no: String,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub notes: String,
}
