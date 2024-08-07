use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct GlobalSettingsModel {
    pub id: i32,
    pub units: Vec<String>,
    pub accounts: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct ColorSettingsModel {
    pub id: i32,
    pub color: String,
    pub value: i32,
    pub create_time: DateTime<Utc>, // 父类ID
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct CustomerTypeModel {
    pub id: i32,
    pub ty_pe: String,
    pub create_time: DateTime<Utc>, // 父类ID
}
