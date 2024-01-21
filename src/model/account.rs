use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct AccountModel {
    pub id: i32,
    pub name: String,
    pub account: String,
    pub password: String,
    pub create_time: DateTime<Utc>,
}
