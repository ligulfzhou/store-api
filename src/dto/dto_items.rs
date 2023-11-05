#[derive(Debug, Deserialize)]
pub struct QueryParams {}

#[derive(Deserialize, Debug)]
pub struct EditParams {}

#[derive(Debug)]
pub struct DeleteParams {
    pub id: i32,
}
