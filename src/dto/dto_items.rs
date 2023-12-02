use crate::dto::dto_account::AccountDto;
use crate::model::embryo::EmbryoModel;
use crate::model::items::ItemsInOutModel;
use chrono::NaiveDateTime;

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct ItemsDto {
    pub id: i32,
    pub images: Vec<String>,        // 商品图片
    pub name: String,               // 产品名称
    pub size: String,               // 规格
    pub color: String,              // 颜色
    pub cate1_id: i32,              // 大类ID
    pub cate1: String,              // 大类名
    pub cate2_id: i32,              // 小类ID
    pub cate2: String,              // 小类名
    pub unit: String,               // 单位
    pub price: i32,                 // 标准售价
    pub cost: i32,                  // 成本
    pub notes: String,              // 备注
    pub number: String,             // 货号
    pub barcode: String,            // 条码
    pub create_time: NaiveDateTime, // 创建时间
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    // todo: more fields
    pub cate1_id: i32,
    pub cate2_id: i32,
    pub name: String,    // 产品名称
    pub number: String,  // 货号
    pub barcode: String, // 货号
    pub create_time_st: String,
    pub create_time_ed: String,

    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

impl QueryParams {
    pub fn is_empty(&self) -> bool {
        if !self.name.is_empty() {
            return false;
        }
        if self.cate1_id != 0 || self.cate2_id != 0 {
            return false;
        }

        if !self.number.is_empty() {
            return false;
        }

        if !self.barcode.is_empty() {
            return false;
        }

        if !self.create_time_ed.is_empty() && !self.create_time_st.is_empty() {
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

//// 出入库 相关

// #[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
// pub struct ItemsInOutModel {
//     pub id: i32,
//     pub account_id: i32,            // 商品图片
//     pub item_id: i32,               // 产品名称
//     pub count: i32,                 // 数量
//     pub in_true_out_false: bool,    // 增加还是减少
//     pub via: String,                // 规格
//     pub order_id: i32,              // 颜色
//     pub create_time: NaiveDateTime, // 创建时间
// }

#[derive(Debug, Serialize)]
pub struct ItemStockDto {
    pub item: ItemsDto,
    pub count: i32,
    pub embryo: Option<EmbryoModel>,
    pub embryo_count: i32,
}

#[derive(Debug, Serialize)]
pub struct ItemInOutDto {
    pub model: ItemsInOutModel,
    pub account: AccountDto,
    pub item: ItemsDto,
    // todo
    pub embryo: EmbryoModel,
}

#[derive(Debug, Deserialize)]
pub struct ItemInOutQueryParams {
    pub account_id: i32,
    pub item_id: i32,
    pub cate1_id: i32,
    pub cate2_id: i32,
    pub number: String,  // 货号
    pub barcode: String, // 货号

    pub page: Option<i32>,
    pub page_size: Option<i32>,
}
