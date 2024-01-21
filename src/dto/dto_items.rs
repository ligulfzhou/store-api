use crate::dto::dto_embryo::EmbryoDto;
use crate::model::embryo::EmbryoModel;
use crate::model::items::{ItemInOutBucketModal, ItemsModel};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Clone, sqlx::FromRow)]
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
    pub count: i32,                 // 库存数
    pub create_time: DateTime<Utc>, // 创建时间
    pub embryo: Option<EmbryoDto>,
}

impl ItemsDto {
    pub fn from(
        item: ItemsModel,
        count: i32,
        cate1: &str,
        cate2: &str,
        embryo: Option<EmbryoDto>,
    ) -> Self {
        Self {
            id: item.id,
            images: item.images,
            name: item.name,
            size: item.size,
            color: item.color,
            cate1_id: item.cate1_id,
            cate1: cate1.to_string(),
            cate2_id: item.cate2_id,
            cate2: cate2.to_string(),
            unit: item.unit,
            price: item.price,
            cost: item.cost,
            notes: item.notes,
            number: item.number,
            barcode: item.barcode,
            count,
            create_time: item.create_time,
            embryo,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct ItemInOutBucketDto {
    pub id: i32,
    pub account_id: i32,            // 经手账号id
    pub account: String,            // 经手账号 名
    pub in_true_out_false: bool,    // 增加还是减少
    pub via: String,                // 规格
    pub create_time: DateTime<Utc>, // 创建时间

    pub total_count: i32,
    pub total_sum: i32,

    pub items: Vec<ItemInOutDto>, // todo: 可能没必要再搞一个没哟accout 名字的struct 出来
}

impl ItemInOutBucketDto {
    pub fn from(
        item_inout_bucket: ItemInOutBucketModal,
        account_name: &str,
        items: Vec<ItemInOutDto>,
        total_count: i32,
        total_sum: i32,
    ) -> Self {
        Self {
            id: item_inout_bucket.id,
            account_id: item_inout_bucket.account_id,
            account: account_name.to_string(),
            in_true_out_false: item_inout_bucket.in_true_out_false,
            via: item_inout_bucket.via,
            create_time: item_inout_bucket.create_time,
            total_count,
            total_sum,
            items,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct SingleItemInOutDto {
    pub id: i32,
    pub account_id: i32,            // 经手账号id
    pub account: String,            // 经手账号 名
    pub item_id: i32,               // 产品名称
    pub count: i32,                 // 数量
    pub in_true_out_false: bool,    // 增加还是减少
    pub via: String,                // 规格
    pub create_time: DateTime<Utc>, // 创建时间
    pub item: Option<ItemsModel>,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct ItemInOutDto {
    pub id: i32,
    pub bucket_id: i32,
    pub item_id: i32,
    pub count: i32,
    pub current_cost: i32,
    pub current_total: i32,

    pub item_name: String,
    pub number: String,
    pub barcode: String,
    pub unit: String,
    pub account_id: i32,         // 经手账号id
    pub account: String,         // 经手账号 名
    pub in_true_out_false: bool, // 增加还是减少
    pub via: String,             // 规格
    pub order_id: i32,
    pub create_time: DateTime<Utc>, // 创建时间
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

#[derive(Deserialize, Debug)]
pub struct InoutBucketParams {
    // pub item_id: i32,
    pub in_out: Option<bool>,

    // todo: 这两个是需要的
    // pub create_time_st: String,
    // pub create_time_ed: String,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

impl InoutBucketParams {
    pub fn is_empty(&self) -> bool {
        // if self.item_id != 0 {
        //     return false;
        // }

        if self.in_out.is_some() {
            return false;
        }
        // if self.create_time_ed.is_empty() && self.create_time_st.is_empty() {
        //     return false;
        // }
        true
    }
}

#[derive(Deserialize, Debug)]
pub struct InoutListOfBucketParams {
    pub bucket_id: i32,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct InoutParams {
    pub id: i32,
    pub in_out: bool,
    pub count: i32,
}

#[derive(Deserialize, Debug)]
pub struct InoutQueryParams {
    pub item_id: i32,

    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ItemSearchParams {
    pub barcode: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemStockOutItem {
    pub item_id: i32,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct ItemStockOutMultiParams {
    pub items: Vec<ItemStockOutItem>,
}
