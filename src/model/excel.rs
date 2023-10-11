#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct CustomerExcelTemplateModel {
    pub id: i32,             // SERIAL,
    pub customer_no: String, // 客户编号
    pub template_id: i32,    // 备注
}
