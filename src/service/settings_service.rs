use crate::config::database::{Database, DatabaseTrait};
use crate::dto::dto_settings::ColorEditParams;
use crate::model::settings::ColorSettingsModel;
use crate::{ERPError, ERPResult};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct SettingsService {
    pub db: Arc<Database>,
}

#[async_trait]
pub trait SettingsServiceTrait {
    fn new(db: &Arc<Database>) -> Self;

    async fn get_all_color_to_values(&self) -> ERPResult<Vec<ColorSettingsModel>>;
    async fn edit_color_to_value(&self, params: &ColorEditParams) -> ERPResult<()>;
}
#[async_trait]
impl SettingsServiceTrait for SettingsService {
    fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }

    async fn get_all_color_to_values(&self) -> ERPResult<Vec<ColorSettingsModel>> {
        let css = sqlx::query_as!(
            ColorSettingsModel,
            "select * from color_settings order by value;"
        )
        .fetch_all(self.db.get_pool())
        .await?;

        Ok(css)
    }

    async fn edit_color_to_value(&self, params: &ColorEditParams) -> ERPResult<()> {
        let existing = self.get_all_color_to_values().await?;
        match params.id {
            0 => {
                // 检查 颜色 是否已经存在
                if existing
                    .iter()
                    .filter(|&item| item.color == params.color)
                    .collect::<Vec<&ColorSettingsModel>>()
                    .len()
                    > 0
                {
                    return Err(ERPError::AlreadyExists(format!(
                        "颜色为'{}'的已经存在",
                        params.color
                    )));
                }

                // 检查 数值 是否已经存在
                if existing
                    .iter()
                    .filter(|&item| item.value == params.value)
                    .collect::<Vec<&ColorSettingsModel>>()
                    .len()
                    > 0
                {
                    return Err(ERPError::AlreadyExists(format!(
                        "值为{:?}的已经存在",
                        params.value
                    )));
                }

                sqlx::query!(
                    "insert into color_settings (color, value) values ($1, $2)",
                    params.color,
                    params.value
                )
                .execute(self.db.get_pool())
                .await?;
            }
            _ => {
                if existing
                    .iter()
                    .filter(|&item| item.color == params.color && item.id != params.id)
                    .collect::<Vec<&ColorSettingsModel>>()
                    .len()
                    > 0
                {
                    return Err(ERPError::AlreadyExists(format!(
                        "颜色为{:?}的已经存在",
                        params.color
                    )));
                }

                // 检查 数值 是否已经存在
                if existing
                    .iter()
                    .filter(|&item| item.value == params.value && item.id != params.id)
                    .collect::<Vec<&ColorSettingsModel>>()
                    .len()
                    > 0
                {
                    return Err(ERPError::AlreadyExists(format!(
                        "值为{:?}的已经存在",
                        params.value
                    )));
                }

                sqlx::query!(
                    "update color_settings set color=$1, value=$2 where id = $3",
                    params.color,
                    params.value,
                    params.id
                )
                .execute(self.db.get_pool())
                .await?;
            }
        }

        Ok(())
    }
}
