use crate::config::database::{Database, DatabaseTrait};
use crate::dto::dto_settings::{ColorEditParams, GlobalSettingsUpdateParams};
use crate::dto::GenericDeleteParams;
use crate::model::settings::{ColorSettingsModel, GlobalSettingsModel};
use crate::{ERPError, ERPResult};
use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder};
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
    async fn delete_color_to_value(&self, params: &GenericDeleteParams) -> ERPResult<()>;
    async fn get_global_settings(&self) -> ERPResult<GlobalSettingsModel>;
    async fn update_global_settings(&self, params: &GlobalSettingsUpdateParams) -> ERPResult<()>;
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
                if !existing
                    .iter()
                    .filter(|&item| item.color == params.color)
                    .collect::<Vec<&ColorSettingsModel>>()
                    .is_empty()
                {
                    return Err(ERPError::AlreadyExists(format!(
                        "颜色为'{}'的已经存在",
                        params.color
                    )));
                }

                // 检查 数值 是否已经存在
                if !existing
                    .iter()
                    .filter(|&item| item.value == params.value)
                    .collect::<Vec<&ColorSettingsModel>>()
                    .is_empty()
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
                if !existing
                    .iter()
                    .filter(|&item| item.color == params.color && item.id != params.id)
                    .collect::<Vec<&ColorSettingsModel>>()
                    .is_empty()
                {
                    return Err(ERPError::AlreadyExists(format!(
                        "颜色为{:?}的已经存在",
                        params.color
                    )));
                }

                // 检查 数值 是否已经存在
                if !existing
                    .iter()
                    .filter(|&item| item.value == params.value && item.id != params.id)
                    .collect::<Vec<&ColorSettingsModel>>()
                    .is_empty()
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

    async fn delete_color_to_value(&self, params: &GenericDeleteParams) -> ERPResult<()> {
        sqlx::query!("delete from color_settings where id = $1", params.id)
            .execute(self.db.get_pool())
            .await?;

        Ok(())
    }

    async fn get_global_settings(&self) -> ERPResult<GlobalSettingsModel> {
        let global_settings = sqlx::query_as!(
            GlobalSettingsModel,
            "select * from global_settings order by id limit 1"
        )
        .fetch_one(self.db.get_pool())
        .await?;

        Ok(global_settings)
    }

    async fn update_global_settings(&self, params: &GlobalSettingsUpdateParams) -> ERPResult<()> {
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new("update global_settings set ");
        if params.units.is_some() {
            sql.push("units=")
                .push_bind(params.units.as_ref())
                .push(",");
        }
        if params.accounts.is_some() {
            sql.push("accounts=").push_bind(params.accounts.as_ref());
        }

        sql.build().execute(self.db.get_pool()).await?;

        Ok(())
    }
}
