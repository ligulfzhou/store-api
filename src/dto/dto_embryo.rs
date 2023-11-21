#[derive(Debug, Deserialize)]
pub struct QueryParams {
    // todo: more fields
    pub color: String,
    pub number: String, // 货号
    pub name: String,   // 产品名称

    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

impl QueryParams {
    pub fn is_empty(&self) -> bool {
        if !self.number.is_empty() {
            return false;
        }
        if !self.name.is_empty() {
            return false;
        }
        if !self.color.is_empty() {
            return false;
        }

        true
    }
}

#[derive(Deserialize, Debug)]
pub struct EditParams {
    pub id: i32,
    pub images: Vec<String>,
    pub name: String,
    pub color: String,
    pub unit: String,
    pub notes: String,
    pub number: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteParams {
    pub id: i32,
}
