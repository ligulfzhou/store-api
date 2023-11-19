use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct EmbryoModel {
    pub id: i32,
    pub images: Vec<String>,
    pub name: String,
    pub color: String,
    pub unit: String,
    pub notes: String,
    pub number: String,
    pub create_time: NaiveDateTime,
}
