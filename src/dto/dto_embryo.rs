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
}

#[derive(Deserialize, Debug)]
pub struct EditParams {
    pub id: i32,
    pub images: Vec<String>, // 商品图片
    pub name: String,        // 产品名称
    pub size: String,        // 规格
    pub color: String,       // 颜色
    pub cate1_id: i32,       // 大类ID
    pub cate2_id: i32,       // 小类ID
    pub unit: String,        // 单位
    pub price: i32,          // 标准售价
    pub cost: i32,           // 成本
    pub notes: String,       // 备注
    pub number: String,      // 货号
    pub barcode: String,     // 条码
}

#[derive(Debug, Deserialize)]
pub struct DeleteParams {
    pub id: i32,
}
