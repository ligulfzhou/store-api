use crate::config::database::{Database, DatabaseTrait};
use crate::dto::dto_settings::{
    ColorEditParams, CustomerTypeEditParams, GlobalSettingsUpdateParams,
};
use crate::dto::GenericDeleteParams;
use crate::model::settings::{ColorSettingsModel, CustomerTypeModel, GlobalSettingsModel};
use crate::{ERPError, ERPResult};
use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder};
use std::collections::HashMap;
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
    async fn add_multiple_color_to_value(
        &self,
        colors: Vec<String>,
    ) -> ERPResult<HashMap<String, i32>>;
    async fn get_global_settings(&self) -> ERPResult<GlobalSettingsModel>;
    async fn update_global_settings(&self, params: &GlobalSettingsUpdateParams) -> ERPResult<()>;

    async fn get_customer_types(&self) -> ERPResult<Vec<CustomerTypeModel>>;
    async fn edit_customer_type(&self, params: &CustomerTypeEditParams) -> ERPResult<()>;
    async fn delete_customer_type(&self, params: &GenericDeleteParams) -> ERPResult<()>;
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
                // if !existing
                //     .iter()
                //     .filter(|&item| item.value == params.value)
                //     .collect::<Vec<&ColorSettingsModel>>()
                //     .is_empty()
                // {
                //     return Err(ERPError::AlreadyExists(format!(
                //         "值为{:?}的已经存在",
                //         params.value
                //     )));
                // }

                sqlx::query!(
                    "insert into color_settings (color, value) values ($1, $2)",
                    params.color.to_ascii_uppercase(),
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
                // if !existing
                //     .iter()
                //     .filter(|&item| item.value == params.value && item.id != params.id)
                //     .collect::<Vec<&ColorSettingsModel>>()
                //     .is_empty()
                // {
                //     return Err(ERPError::AlreadyExists(format!(
                //         "值为{:?}的已经存在",
                //         params.value
                //     )));
                // }

                sqlx::query!(
                    "update color_settings set color=$1, value=$2 where id = $3",
                    params.color.to_ascii_uppercase(),
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

    async fn add_multiple_color_to_value(
        &self,
        colors: Vec<String>,
    ) -> ERPResult<HashMap<String, i32>> {
        let mut max = sqlx::query!("select max(value) from color_settings")
            .fetch_one(self.db.get_pool())
            .await?
            .max
            .unwrap_or(0) as i32;

        let mut color_models = vec![];
        for color in colors {
            color_models.push(ColorSettingsModel {
                id: 0,
                color,
                value: max + 1,
                create_time: Default::default(),
            });
            max += 1;
        }

        if !color_models.is_empty() {
            let mut query_builder: QueryBuilder<Postgres> =
                QueryBuilder::new("insert into color_settings (color, value) ");

            query_builder.push_values(color_models, |mut b, item| {
                b.push_bind(item.color).push_bind(item.value);
            });

            query_builder.push(" returning *;");

            Ok(query_builder
                .build_query_as::<ColorSettingsModel>()
                .fetch_all(self.db.get_pool())
                .await?
                .into_iter()
                .map(|item| (item.color, item.value))
                .collect::<HashMap<_, _>>())
        } else {
            Ok(HashMap::new())
        }
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

    async fn get_customer_types(&self) -> ERPResult<Vec<CustomerTypeModel>> {
        let customer_types = sqlx::query_as!(
            CustomerTypeModel,
            "select * from customer_types order by id"
        )
        .fetch_all(self.db.get_pool())
        .await?;

        Ok(customer_types)
    }

    async fn edit_customer_type(&self, params: &CustomerTypeEditParams) -> ERPResult<()> {
        let customer_types = self.get_customer_types().await?;
        if !customer_types
            .iter()
            .filter(|item| item.id != params.id && item.ty_pe == params.ty_pe)
            .collect::<Vec<_>>()
            .is_empty()
        {
            return Err(ERPError::AlreadyExists(format!(
                "客户类型名: {} 已存在",
                params.ty_pe
            )));
        }

        match params.id {
            0 => {
                // insert
                sqlx::query!(
                    "insert into customer_types (ty_pe) values ($1)",
                    params.ty_pe
                )
                .execute(self.db.get_pool())
                .await?;
            }
            _ => {
                // update
                sqlx::query!(
                    "update customer_types set ty_pe=$1 where id=$2",
                    params.ty_pe,
                    params.id
                )
                .execute(self.db.get_pool())
                .await?;
            }
        }

        Ok(())
    }

    async fn delete_customer_type(&self, params: &GenericDeleteParams) -> ERPResult<()> {
        let _ = sqlx::query_as!(
            CustomerTypeModel,
            "select * from customer_types where id=$1",
            params.id
        )
        .fetch_one(self.db.get_pool())
        .await
        .map_err(|_err| ERPError::NotFound("数据不存在，请刷新".to_string()));

        if sqlx::query!("select count(1) from customers where id = $1", params.id)
            .fetch_one(self.db.get_pool())
            .await?
            .count
            .unwrap_or(0)
            > 0
        {
            return Err(ERPError::Failed("删除不合法，有对应的客户存在".to_string()));
        }

        sqlx::query!("delete from customer_types where id = $1", params.id)
            .execute(self.db.get_pool())
            .await?;

        Ok(())
    }
}
