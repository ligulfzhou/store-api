use crate::constants::SORTER_ORDER_TO_DB_SORTER_ORDER;

pub fn sorter_order_to_db_sorter_order(order: &str) -> &'static str {
    SORTER_ORDER_TO_DB_SORTER_ORDER
        .get(order)
        .unwrap_or(&"desc")
}
