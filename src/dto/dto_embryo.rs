use crate::model::embryo::{EmbryoInOutBucketModal, EmbryoModel};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// model => dto
#[derive(Debug, Serialize, Clone)]
pub struct EmbryoDto {
    pub id: i32,
    pub images: Vec<String>,
    pub name: String,
    pub color: String,
    pub unit: String,
    pub notes: String,
    pub number: String,
    pub create_time: DateTime<Utc>,
    pub count: i32,
}

impl EmbryoDto {
    pub fn from(embryo: EmbryoModel, count: i32) -> Self {
        Self {
            id: embryo.id,
            images: embryo.images,
            name: embryo.name,
            color: embryo.color,
            unit: embryo.unit,
            notes: embryo.notes,
            number: embryo.number,
            create_time: embryo.create_time,
            count,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct EmbryoInOutDto {
    pub id: i32,
    pub bucket_id: i32,
    pub embryo_id: i32,
    pub count: i32,
    pub current_cost: i32,
    pub current_total: i32,

    pub embryo_name: String,
    pub number: String,
    pub unit: String,
    pub account_id: i32,            // 经手账号id
    pub account: String,            // 经手账号 名
    pub in_true_out_false: bool,    // 增加还是减少
    pub via: String,                // 规格
    pub create_time: DateTime<Utc>, // 创建时间
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct EmbryoInOutBucketDto {
    pub id: i32,
    pub account_id: i32,            // 经手账号id
    pub account: String,            // 经手账号 名
    pub in_true_out_false: bool,    // 增加还是减少
    pub via: String,                // 规格
    pub create_time: DateTime<Utc>, // 创建时间

    pub total_count: i32,
    pub total_sum: i32,

    pub items: Vec<EmbryoInOutDto>, // todo: 可能没必要再搞一个没哟accout 名字的struct 出来
}

impl EmbryoInOutBucketDto {
    pub fn from(
        item_inout_bucket: EmbryoInOutBucketModal,
        account_name: &str,
        items: Vec<EmbryoInOutDto>,
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

/// params

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    // pub color: String,
    pub number: String, // 货号
    pub name: String,   // 产品名称

    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

impl QueryParams {
    pub fn is_empty(&self) -> bool {
        if !self.number.is_empty() {
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
    pub images: Vec<String>,
    pub name: String,
    pub color: String,
    pub unit: String,
    pub notes: String,
    pub number: String,
}

#[derive(Deserialize, Debug)]
pub struct InoutParams {
    pub id: i32,
    pub in_out: bool,
    // pub via: String, todo: 应该是不需要，肯定是form
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct InoutListParams {
    pub embryo_id: i32, // 产品名称

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
