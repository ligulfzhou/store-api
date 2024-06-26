use crate::model::cates::CateModel;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CateDto {
    pub id: i32,        // SERIAL
    pub index: i32,     // 顺序
    pub name: String,   // 类名
    pub cate_type: i32, // 大类小类， 0 大类， 1小类，再变大，则更小
    pub parent_id: i32, // 父类ID
    pub create_time: DateTime<Utc>,
    pub sub_cates: Option<Vec<CateDto>>,
}

impl CateDto {
    pub fn from(cate: CateModel, sub_cates: Option<Vec<CateDto>>) -> Self {
        Self {
            id: cate.id,
            index: cate.index,
            name: cate.name,
            cate_type: cate.cate_type,
            parent_id: cate.parent_id,
            create_time: cate.create_time,
            sub_cates,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SubCatesParams {
    pub id: i32,
}

#[derive(Deserialize, Debug)]
pub struct EditParams {
    pub id: i32,        // SERIAL
    pub index: i32,     // 顺序
    pub name: String,   // 类名
    pub cate_type: i32, // 大类小类， 0 大类， 1小类，再变大，则更小
    pub parent_id: i32, // 父类ID
}
