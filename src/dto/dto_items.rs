#[derive(Debug, Deserialize)]
pub struct QueryParams {
    // todo: more fields
    pub brand: String,    // 品牌
    pub cates1: String,   // 产品大类
    pub cates2: String,   // 产品小类
    pub goods_no: String, // 货号
    pub name: String,     // 产品名称

    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

impl QueryParams {
    pub fn is_empty(&self) -> bool {
        if !self.brand.is_empty() {
            return false;
        }
        if !self.cates1.is_empty() || !self.cates2.is_empty() {
            return false;
        }

        if !self.goods_no.is_empty() {
            return false;
        }

        if !self.name.is_empty() {
            return false;
        }

        true
    }
    pub fn to_pagination_sql(&self) -> String {
        todo!()
    }
}

#[derive(Deserialize, Debug)]
pub struct EditParams {}

#[derive(Debug)]
pub struct DeleteParams {
    pub id: i32,
}
