use crate::model::embryo::EmbryoModel;
use chrono::NaiveDateTime;
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
    pub create_time: NaiveDateTime,
    // pub embryo: EmbryoModel,
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
    pub current_price: i32,
    pub current_total: i32,

    pub embryo_name: String,
    pub account_id: i32,            // 经手账号id
    pub account: String,            // 经手账号 名
    pub in_true_out_false: bool,    // 增加还是减少
    pub via: String,                // 规格
    pub create_time: NaiveDateTime, // 创建时间
}

// impl EmbryoInOutDto {
//     pub fn from(
//         embryo_in_out: EmbryoInOutModel,
//         account: &str,
//         embryo: Option<EmbryoModel>,
//     ) -> Self {
//         Self {
//             id: embryo_in_out.id,
//             account_id: embryo_in_out.account_id,
//             account: account.to_string(),
//             embryo_id: embryo_in_out.embryo_id,
//             count: embryo_in_out.count,
//             in_true_out_false: embryo_in_out.in_true_out_false,
//             via: embryo_in_out.via,
//             create_time: embryo_in_out.create_time,
//             embryo,
//         }
//     }
// }

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
