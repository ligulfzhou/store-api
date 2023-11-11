use crate::constants::DEFAULT_PAGE_SIZE;
use crate::model::customer::CustomerModel;
use chrono::NaiveDate;
use std::ops::Deref;

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomerDto {
    pub id: i32,
    pub customer_no: String,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub notes: String,
}

impl CustomerDto {
    pub fn from(customer: CustomerModel) -> CustomerDto {
        Self {
            id: customer.id,
            customer_no: customer.customer_no,
            name: customer.name,
            address: customer.address,
            phone: customer.phone,
            notes: customer.notes,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CustomerSearchParam {
    pub page: Option<i32>,
    pub page_size: Option<i32>,

    pub ty_pe: i32,
    pub name: String,
    pub phone: String,
    pub head: String,
    pub create_time_st: String,
    pub create_time_ed: String,
}

impl CustomerSearchParam {
    pub fn to_pagination_sql(&self) -> String {
        let mut sql = "select * from customers ".to_string();
        let mut where_clauses = vec![];
        if self.ty_pe != 0 {
            where_clauses.push(format!(" ty_pe = {} ", self.ty_pe));
        }
        if !self.name.is_empty() {
            where_clauses.push(format!(" name like '%{}%' ", self.name.deref()));
        }
        if !self.head.is_empty() {
            where_clauses.push(format!(" head like '%{}%'", self.head.deref()));
        }
        if !self.phone.is_empty() {
            where_clauses.push(format!(" phone like '%{}%' ", self.phone.deref()));
        }

        if !self.create_time_st.is_empty() && !self.create_time_ed.is_empty() {
            where_clauses.push(format!(
                " create_time >= '{}' and create_time <= '{}' ",
                self.create_time_st.deref(),
                self.create_time_ed.deref()
            ));
        }

        if where_clauses.len() > 0 {
            sql.push_str(&format!(" where {}", where_clauses.join(" and ")));
        }

        let page = self.page.unwrap_or(1);
        let page_size = self.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        let offset = (page - 1) * page_size;

        sql.push_str(&format!(
            " order by id desc limit {page_size} offset {offset};"
        ));

        sql
    }

    pub fn to_count_sql(&self) -> String {
        let mut sql = "select count(1) from customers ".to_string();
        let mut where_clauses = vec![];
        if self.ty_pe != 0 {
            where_clauses.push(format!(" ty_pe = {} ", self.ty_pe));
        }
        if !self.name.is_empty() {
            where_clauses.push(format!(" name like '%{}%' ", self.name.deref()));
        }
        if !self.head.is_empty() {
            where_clauses.push(format!(" head like '%{}%'", self.head.deref()));
        }
        if !self.phone.is_empty() {
            where_clauses.push(format!(" phone like '%{}%' ", self.phone.deref()));
        }

        if !self.create_time_st.is_empty() && !self.create_time_ed.is_empty() {
            where_clauses.push(format!(
                " create_time >= '{}' and create_time <= '{}' ",
                self.create_time_st.deref(),
                self.create_time_ed.deref()
            ));
        }

        if where_clauses.len() > 0 {
            sql.push_str(&format!(" where {}", where_clauses.join(" and ")));
        }

        sql
    }
}

#[derive(Debug, Deserialize)]
pub struct CustomerEditParam {
    pub id: Option<i32>,
    pub ty_pe: i32,
    pub name: String,
    pub head: String,
    pub address: String,
    pub email: String,
    pub birthday: Option<NaiveDate>,
    pub phone: String,
    pub notes: String,
}

#[derive(Debug, Deserialize)]
pub struct CustomerDeleteParam {
    pub id: i32,
}
