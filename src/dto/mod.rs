pub mod dto_account;
pub mod dto_cates;
pub mod dto_customer;
pub mod dto_excel;
pub mod dto_items;
pub mod dto_orders;
pub mod dto_settings;

#[derive(Deserialize, Debug)]
pub struct GenericDeleteParams {
    pub id: i32,
}
