#[derive(Deserialize, Debug)]
pub struct ColorEditParams {
    pub id: i32,
    pub color: String,
    pub value: i32,
}
