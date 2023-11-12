use chrono::NaiveDateTime;

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct GlobalSettingsModel {
    pub id: i32,
    pub units: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct ColorSettingsModel {
    pub id: i32,
    pub color: String,
    pub value: i32,
    pub create_time: NaiveDateTime, // 父类ID
}
