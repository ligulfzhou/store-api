use std::collections::HashMap;

pub const DEFAULT_PAGE_SIZE: i32 = 50;
pub const DONE_INDEX: i32 = 2;
pub const EXCEPTION_OR_NOTES_INDEX: i32 = 1;

// pub const STORAGE_FILE_PATH: &str = "/home/debian/data/erp";
pub const STORAGE_FILE_PATH: &str = "/Users/ligangzhou/data/erp";

// pub const STORAGE_URL_PREFIX: &str = "https://store-erp.ligulfzhou.com/erp";
pub const STORAGE_URL_PREFIX: &str = "http://localhost/erp";

lazy_static! {
    pub static ref STEP_TO_DEPARTMENT: HashMap<i32, &'static str> =
        vec![(1, "业务部"), (2, "仓库"), (3, "车间"),]
            .into_iter()
            .collect();
    pub static ref SORTER_ORDER_TO_DB_SORTER_ORDER: HashMap<&'static str, &'static str> =
        vec![("descend", "desc"), ("ascend", "asc"),]
            .into_iter()
            .collect();
}
