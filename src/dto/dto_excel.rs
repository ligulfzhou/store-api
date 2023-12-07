#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ItemExcelDto {
    pub images: Vec<String>, // 图片
    pub name: String,        // 名称
    pub color: String,       // 颜色
    pub cates1: String,      // 产品大类
    pub cate1_id: i32,       // 产品大类的ID (由自己算，非导入)
    pub cates2: String,      // 产品小类
    pub cate2_id: i32,       // 产品小类的ID (由自己算，非导入)
    pub number: String,      // 货号
    pub barcode: String,     // 条码
    pub size: String,        // 规格
    pub unit: String,        // 单位
    pub price: i32,          // 标准售价
    pub cost: i32,           // 进货价
    pub notes: String,       // 备注
    pub count: i32,          // 数量
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct EmbryoExcelDto {
    pub images: Vec<String>,
    // 图片
    pub name: String,
    // 名称
    pub color: String,
    // 颜色
    pub number: String,
    // 货号
    pub unit: String,
    // 单位
    pub count: i32,
    // 单价
    pub cost: i32,
    // 数量
    pub notes: String,
}
