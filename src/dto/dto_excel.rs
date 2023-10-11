#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ItemExcelDto {
    pub brand: String,    // 品牌
    pub cates1: String,   // 产品大类
    pub cates2: String,   // 产品小类
    pub goods_no: String, // 货号
    pub color: String,    // 颜色
    pub name: String,     // 产品名称
    pub size: String,     // 规格
    pub unit: String,     // 单位
    pub barcode: String,  // 条码
    pub sell_price: i32,  // 标准售价
    pub buy_price: i32,   // 进货价
}
