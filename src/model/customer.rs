use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct CustomerModel {
    pub id: i32,
    pub ty_pe: i32,
    pub name: String,
    pub head: String,
    pub address: String,
    pub email: String,
    pub birthday: Option<NaiveDate>,
    pub phone: String,
    pub notes: String,
    pub create_time: DateTime<Utc>,
}
