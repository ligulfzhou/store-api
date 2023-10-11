use crate::constants::STEP_TO_DEPARTMENT;
use crate::model::customer::CustomerModel;
use crate::model::order::OrderModel;
use chrono::NaiveDate;
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct OrderDto {
    pub id: i32,
    pub customer_no: String,
    pub order_no: String,
    pub order_date: NaiveDate,
    pub delivery_date: Option<NaiveDate>,
    pub is_return_order: bool,
    pub is_urgent: bool,
}

impl OrderDto {
    pub fn from(order: OrderModel, customer: CustomerModel) -> OrderDto {
        Self {
            id: order.id,
            customer_no: customer.customer_no,
            order_no: order.order_no,
            order_date: order.order_date,
            delivery_date: order.delivery_date,
            is_return_order: order.is_return_order,
            is_urgent: order.is_urgent,
        }
    }

    pub fn from_only(order: OrderModel) -> OrderDto {
        Self {
            id: order.id,
            customer_no: order.customer_no,
            order_no: order.order_no,
            order_date: order.order_date,
            delivery_date: order.delivery_date,
            is_return_order: order.is_return_order,
            is_urgent: order.is_urgent,
        }
    }
}

type StepCount = HashMap<i32, i32>;
type StepCountUF = HashMap<String, i32>;

type StepIndexCount = HashMap<(i32, i32), i32>;

#[derive(Debug, Serialize)]
pub struct StepIndexCountUF {
    pub step: i32,
    pub index: i32,
    pub count: i32,
}

impl StepIndexCountUF {
    pub fn from_step_index_count(step_index_count: StepIndexCount) -> Vec<StepIndexCountUF> {
        let mut ufs = step_index_count
            .iter()
            .map(|kv| StepIndexCountUF {
                step: kv.0 .0,
                index: kv.0 .1,
                count: *kv.1,
            })
            .collect::<Vec<StepIndexCountUF>>();

        ufs.sort_by_key(|kv| (kv.step, kv.index));

        ufs
    }
}

pub fn to_step_count_user_friendly(sc: StepCount) -> StepCountUF {
    sc.into_iter()
        .map(|item| {
            (
                STEP_TO_DEPARTMENT.get(&item.0).unwrap_or(&"").to_string(),
                item.1,
            )
        })
        .collect::<HashMap<String, i32>>()
}

#[derive(Debug, Serialize)]
pub struct OrderWithStepsDto {
    pub id: i32,
    pub customer_no: String,
    pub order_no: String,
    pub order_date: NaiveDate,
    pub delivery_date: Option<NaiveDate>,
    pub is_return_order: bool,
    pub is_urgent: bool,
    pub steps: Vec<StepIndexCountUF>,
}

impl OrderWithStepsDto {
    pub fn from_order_dto_and_steps(order: OrderDto, steps: StepIndexCount) -> OrderWithStepsDto {
        Self {
            id: order.id,
            customer_no: order.customer_no,
            order_no: order.order_no,
            order_date: order.order_date,
            delivery_date: order.delivery_date,
            is_return_order: order.is_return_order,
            is_urgent: order.is_urgent,
            steps: StepIndexCountUF::from_step_index_count(steps),
        }
    }
}

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct OrderGoodsItemDto {
    pub id: i32,
    pub order_id: i32,
    pub order_goods_id: i32,
    pub goods_id: i32,
    pub sku_id: i32,
    pub sku_no: Option<String>,
    pub color: String,
    pub count: i32,
    pub unit: Option<String>,
    pub unit_price: Option<i32>,
    pub total_price: Option<i32>,
    pub notes: String,
}

#[derive(Debug, Serialize, Clone, FromRow)]
pub struct OrderPlainItemDto {
    pub id: i32,
    pub order_id: i32,
    pub goods_id: i32,
    pub goods_no: String,
    pub name: String,
    pub image: String,
    pub package_card: String,
    pub package_card_des: String,
    pub order_goods_id: i32,
    pub sku_id: i32,
    pub sku_no: Option<String>,
    pub color: String,
    pub count: i32,
    pub unit: Option<String>,
    pub unit_price: Option<i32>,
    pub total_price: Option<i32>,
    pub notes: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct OrderPlainItemWithCurrentStepDto {
    pub id: i32,
    pub order_id: i32,
    pub goods_id: i32,
    pub goods_no: String,
    pub name: String,
    pub image: String,
    pub package_card: String,
    pub package_card_des: String,
    pub order_goods_id: i32,
    pub sku_id: i32,
    pub sku_no: Option<String>,
    pub color: String,
    pub count: i32,
    pub unit: Option<String>,
    pub unit_price: Option<i32>,
    pub total_price: Option<i32>,
    pub notes: String,

    pub is_next_action: bool,
    pub current_step: i32,
    pub current_index: i32,
    pub current_notes: String,
    pub step: String,
}

impl OrderPlainItemWithCurrentStepDto {
    pub fn from(
        item: OrderPlainItemDto,
        is_next_action: bool,
        current_step: i32,
        current_index: i32,
        current_notes: &str,
    ) -> OrderPlainItemWithCurrentStepDto {
        Self {
            id: item.id,
            order_id: item.order_id,
            goods_id: item.goods_id,
            goods_no: item.goods_no,
            name: item.name,
            image: item.image,
            package_card: item.package_card,
            package_card_des: item.package_card_des,
            order_goods_id: item.order_goods_id,
            sku_id: item.sku_id,
            sku_no: item.sku_no,
            color: item.color,
            count: item.count,
            unit: item.unit,
            unit_price: item.unit_price,
            total_price: item.total_price,
            notes: item.notes,
            is_next_action,
            current_step,
            current_index,
            step: STEP_TO_DEPARTMENT
                .get(&current_step)
                .unwrap_or(&"")
                .to_string(),
            current_notes: current_notes.to_string(),
        }
    }
}

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct OrderGoodsDto {
    pub id: i32,
    pub order_id: i32,
    pub goods_id: i32,
    pub goods_no: String,
    pub name: String,
    pub image: String,
    pub plating: String,
    pub package_card: String,
    pub package_card_des: String,
}

#[derive(Debug, Serialize)]
struct OrderItemDto {
    id: i32,
    order_id: i32, // -- 订单ID
    sku_id: i32,   // integer not null, -- 商品ID
    // order_goods_id: i32,   // integer not null,
    package_card: String,     // text,    -- 包装卡片    （存在大问题）
    package_card_des: String, //  -- 包装卡片说明 （存在大问题）
    count: i32,               //   integer not null,  - - 数量
    unit: String,             //  text,- - 单位
    unit_price: Option<i32>,  //  integer, - - 单价
    total_price: Option<i32>, //   integer,  - - 总价 / 金额
    notes: String,            //    text - - 备注,
}
