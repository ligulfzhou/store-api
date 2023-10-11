pub mod routes_account;
pub mod routes_customer;
pub mod routes_excel;
pub mod routes_login;
pub mod routes_material;
pub mod routes_static;
pub mod routes_upload;

pub trait ListParamToSQLTrait {
    fn to_pagination_sql(&self) -> String;
    fn to_count_sql(&self) -> String;
}

pub trait CreateOrUpdateParamToSQLTrait {
    fn to_sql(&self) -> String;
}
