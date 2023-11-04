use crate::model::cates::CateModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CateDto {
    pub id: i32,                // SERIAL
    pub index: i32,             // 顺序
    pub name: String,           // 类名
    pub sub_cates: Vec<String>, // 子类
}

impl CateDto {
    pub fn from(cate: CateModel) -> Self {
        Self {
            id: cate.id,
            index: cate.index,
            name: cate.name,
            sub_cates: cate.sub_cates,
        }
    }
}
