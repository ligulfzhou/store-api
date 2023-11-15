#[derive(Deserialize, Debug)]
pub struct ColorEditParams {
    pub id: i32,
    pub color: String,
    pub value: i32,
}

#[derive(Deserialize, Debug)]
pub struct GlobalSettingsUpdateParams {
    pub units: Option<Vec<String>>,
    pub accounts: Option<Vec<String>>,
}
