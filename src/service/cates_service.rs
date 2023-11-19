use crate::config::database::{Database, DatabaseTrait};
use crate::dto::dto_cates::{CateDto, EditParams};
use crate::dto::GenericDeleteParams;
use crate::model::cates::CateModel;
use crate::{ERPError, ERPResult};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct CateService {
    pub db: Arc<Database>,
}

#[async_trait]
pub trait CateServiceTrait {
    fn new(db: &Arc<Database>) -> Self;
    async fn get_all_cates(&self) -> ERPResult<Vec<CateDto>>;

    async fn get_sub_cates_of(&self, parent_id: i32) -> ERPResult<Vec<CateDto>>;

    async fn edit_cates(&self, params: &EditParams) -> ERPResult<()>;

    async fn extract_cates(&self) -> ERPResult<()>;

    async fn delete_cate(&self, params: &GenericDeleteParams) -> ERPResult<()>;
}

#[async_trait]
impl CateServiceTrait for CateService {
    fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }

    async fn get_all_cates(&self) -> ERPResult<Vec<CateDto>> {
        let cate_dtos = sqlx::query_as!(CateModel, "select * from cates order by index, id;")
            .fetch_all(self.db.get_pool())
            .await?
            .into_iter()
            .map(|item| CateDto::from(item, None))
            .collect::<Vec<CateDto>>();

        let mut parent_id_to_cates = HashMap::new();
        for cate in cate_dtos {
            parent_id_to_cates
                .entry(cate.parent_id)
                .or_insert(vec![])
                .push(cate);
        }

        let empty_cates: Vec<CateDto> = vec![];
        let parent_cates = parent_id_to_cates.get(&0).unwrap_or(&empty_cates);

        let cates = parent_cates
            .iter()
            .map(|parent_cate| {
                let mut parent = parent_cate.clone();
                let sub_cates = parent_id_to_cates
                    .get(&parent_cate.id)
                    .unwrap_or(&empty_cates)
                    .clone();
                parent.sub_cates = Some(sub_cates);
                parent
            })
            .collect::<Vec<CateDto>>();

        Ok(cates)
    }
    async fn extract_cates(&self) -> ERPResult<()> {
        todo!()
    }

    async fn edit_cates(&self, params: &EditParams) -> ERPResult<()> {
        let cates = self.get_all_cates().await?;

        // checking collision
        match params.cate_type {
            0 => {
                let names = cates
                    .iter()
                    .filter_map(|item| {
                        if item.id == params.id {
                            None
                        } else {
                            Some(&item.name)
                        }
                    })
                    .collect::<Vec<_>>();

                if names.contains(&&params.name) {
                    return Err(ERPError::AlreadyExists(format!(
                        "大类{}已存在",
                        params.name
                    )));
                }
            }
            _ => {
                let pcates = cates
                    .iter()
                    .filter(|item| item.id == params.parent_id)
                    .collect::<Vec<&CateDto>>();

                if pcates.is_empty() {
                    return Err(ERPError::NotFound("大类未找到".to_string()));
                }

                let pcate = pcates[0];
                let names = match &pcate.sub_cates {
                    None => vec![],
                    Some(sub_cates) => sub_cates
                        .iter()
                        .filter_map(|item| {
                            if params.id == item.id {
                                None
                            } else {
                                Some(&item.name)
                            }
                        })
                        .collect::<Vec<_>>(),
                };

                if names.contains(&&params.name) {
                    return Err(ERPError::AlreadyExists(format!(
                        "小类{}已存在",
                        params.name
                    )));
                }
            }
        };

        // todo
        match params.id {
            0 => {
                // 新增item
                sqlx::query!(
                    r#"
                    insert into cates (index, name, cate_type, parent_id)
                    values ($1, $2, $3, $4);
                    "#,
                    params.index,
                    params.name,
                    params.cate_type,
                    params.parent_id,
                )
                .execute(self.db.get_pool())
                .await?;
            }
            _ => {
                // 修改item
                let cate =
                    sqlx::query_as!(CateModel, "select * from cates where id = $1", params.id)
                        .fetch_one(self.db.get_pool())
                        .await?;
                match cate.cate_type {
                    1 => {}
                    _ => {
                        // sqlx::query_as!(CateModel, "select * from cates where ")
                    }
                }

                sqlx::query!(
                    r#"
                    update cates set index=$1, name=$2, cate_type=$3, parent_id=$4
                    where id=$5
                    "#,
                    params.index,
                    params.name,
                    params.cate_type,
                    params.parent_id,
                    params.id,
                )
                .execute(self.db.get_pool())
                .await?;
            }
        };

        Ok(())
    }

    async fn delete_cate(&self, params: &GenericDeleteParams) -> ERPResult<()> {
        let cate = sqlx::query_as!(CateModel, "select * from cates where id = $1", params.id)
            .fetch_one(self.db.get_pool())
            .await?;

        // 检查产品是否存在
        match cate.cate_type {
            0 => {
                let count = sqlx::query!("select count(1) from items where cate1_id = $1", cate.id)
                    .fetch_one(self.db.get_pool())
                    .await?
                    .count
                    .unwrap_or(0) as i32;
                if count > 0 {
                    return Err(ERPError::Failed(format!(
                        "删除不合法, 大类为{}的商品还有{}个",
                        cate.name, count
                    )));
                }
            }
            _ => {
                let count = sqlx::query!("select count(1) from items where cate2_id = $1", cate.id)
                    .fetch_one(self.db.get_pool())
                    .await?
                    .count
                    .unwrap_or(0) as i32;
                if count > 0 {
                    return Err(ERPError::Failed(format!(
                        "删除不合法, 小类为{}的商品还有{}个",
                        cate.name, count
                    )));
                }
            }
        }

        sqlx::query!("delete from cates where id = $1", params.id)
            .execute(self.db.get_pool())
            .await?;

        Ok(())
    }

    async fn get_sub_cates_of(&self, parent_id: i32) -> ERPResult<Vec<CateDto>> {
        let cates = sqlx::query_as!(
            CateModel,
            "select * from cates where parent_id = $1 order by index, id;",
            parent_id
        )
        .fetch_all(self.db.get_pool())
        .await?
        .into_iter()
        .map(|item| CateDto::from(item, None))
        .collect::<Vec<CateDto>>();

        Ok(cates)
    }
}
