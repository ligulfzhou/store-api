#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct AccountModel {
    pub id: i32,
    pub name: String,
    pub account: String,
    pub password: String,
    pub department_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct DepartmentModel {
    pub id: i32,
    pub name: String,
    pub steps: Vec<i32>,
}
