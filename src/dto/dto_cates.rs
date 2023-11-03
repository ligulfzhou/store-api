use crate::model::cates::CateModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CateDto {
    pub cate: CateModel,
    pub sub_cates: Vec<CateModel>,
}
