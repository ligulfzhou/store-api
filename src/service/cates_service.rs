use crate::config::database::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct CateService {
    db: Arc<Database>,
}

pub trait CateServiceTrait {
    fn new(db: &Arc<Database>) -> Self;
}

impl CateServiceTrait for CateService {
    fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }
}

// goods_images_and_package related
// impl CatesService {
//     pub async fn get_goods_images_and_package(
//         db: &Pool<Postgres>,
//         goods_id: i32,
//     ) -> ERPResult<GoodsImagesAndPackage> {
//         let goods_images_package = sqlx::query_as!(
//             GoodsImagesAndPackage,
//             r#"
//             select goods_id, images, image_des, package_card, package_card_des
//             from order_goods
//             where goods_id=$1
//             order by id desc
//             limit 1
//             "#,
//             goods_id
//         )
//         .fetch_optional(db)
//         .await
//         .map_err(ERPError::DBError)?
//         .ok_or(ERPError::NotFound("有商品未找到".to_string()))?;
//
//         Ok(goods_images_package)
//     }
//
//     pub async fn get_multiple_goods_images_and_package(
//         db: &Pool<Postgres>,
//         goods_ids: &[i32],
//     ) -> ERPResult<Vec<GoodsImagesAndPackage>> {
//         let goods_images_package = sqlx::query_as!(
//             GoodsImagesAndPackage,
//             r#"
//             select distinct on (goods_id)
//             goods_id, images, image_des, package_card, package_card_des
//             from order_goods
//             where goods_id = any($1)
//             order by goods_id desc, id desc;
//             "#,
//             goods_ids
//         )
//         .fetch_all(db)
//         .await
//         .map_err(ERPError::DBError)?;
//
//         Ok(goods_images_package)
//     }
// }
//
// // goods related
// impl GoodsService {
//     pub async fn get_goods_dtos(
//         db: &Pool<Postgres>,
//         goods_ids: &[i32],
//     ) -> ERPResult<Vec<GoodsDto>> {
//         let goods_without_images_package = sqlx::query_as!(
//             GoodsModel,
//             "select * from goods where id = any($1)",
//             goods_ids
//         )
//         .fetch_all(db)
//         .await
//         .map_err(ERPError::DBError)?;
//
//         let goods_id_to_images_package =
//             GoodsService::get_multiple_goods_images_and_package(db, &goods_ids)
//                 .await?
//                 .into_iter()
//                 .map(|item| (item.goods_id, item))
//                 .collect::<HashMap<i32, GoodsImagesAndPackage>>();
//
//         let default_images_package = GoodsImagesAndPackage::default();
//
//         let mut goodses = vec![];
//         for goods in goods_without_images_package.into_iter() {
//             let images_package = goods_id_to_images_package
//                 .get(&goods.id)
//                 .unwrap_or(&default_images_package);
//             goodses.push(GoodsDto::from_goods_and_images_package(
//                 goods,
//                 images_package.clone(),
//             ));
//         }
//
//         Ok(goodses)
//     }
// }
//
// // sku related
// impl GoodsService {
//     pub async fn get_sku_dtos(db: &Pool<Postgres>, sku_ids: &[i32]) -> ERPResult<Vec<SKUModelDto>> {
//         let skus_no_image_package = sqlx::query_as!(
//             SKUModelWithoutImageAndPackageDto,
//             r#"
//             select
//                 s.id, s.sku_no, g.customer_no, g.name, g.goods_no, g.id as goods_id,
//                 g.plating, s.color, s.color2, s.notes
//             from skus s, goods g
//             where s.goods_id = g.id
//                 and s.id = any($1)
//             "#,
//             sku_ids
//         )
//         .fetch_all(db)
//         .await
//         .map_err(ERPError::DBError)?;
//
//         let mut goods_ids = skus_no_image_package
//             .iter()
//             .map(|item| item.goods_id)
//             .collect::<Vec<i32>>();
//         goods_ids.dedup();
//
//         let goods_id_to_images_package =
//             GoodsService::get_multiple_goods_images_and_package(db, &goods_ids)
//                 .await?
//                 .into_iter()
//                 .map(|item| (item.goods_id, item))
//                 .collect::<HashMap<i32, GoodsImagesAndPackage>>();
//
//         let default_images_package = GoodsImagesAndPackage::default();
//
//         let mut skus = vec![];
//         for sku in skus_no_image_package.into_iter() {
//             let images_package = goods_id_to_images_package
//                 .get(&sku.goods_id)
//                 .unwrap_or(&default_images_package);
//             skus.push(SKUModelDto::from_sku_and_images_package(
//                 sku,
//                 images_package.clone(),
//             ));
//         }
//
//         Ok(skus)
//     }
//
//     pub async fn get_sku_dtos_with_goods_ids(
//         db: &Pool<Postgres>,
//         goods_ids: &[i32],
//     ) -> ERPResult<Vec<SKUModelDto>> {
//         let skus_no_image_package = sqlx::query_as!(
//             SKUModelWithoutImageAndPackageDto,
//             r#"
//             select
//                 s.id, s.sku_no, g.customer_no, g.name, g.goods_no, g.id as goods_id,
//                 g.plating, s.color, s.color2, s.notes
//             from skus s, goods g
//             where s.goods_id = g.id
//                 and s.goods_id = any($1)
//             "#,
//             goods_ids
//         )
//         .fetch_all(db)
//         .await
//         .map_err(ERPError::DBError)?;
//
//         let goods_id_to_images_package =
//             GoodsService::get_multiple_goods_images_and_package(db, goods_ids)
//                 .await?
//                 .into_iter()
//                 .map(|item| (item.goods_id, item))
//                 .collect::<HashMap<i32, GoodsImagesAndPackage>>();
//
//         let default_images_package = GoodsImagesAndPackage::default();
//
//         let mut skus = vec![];
//         for sku in skus_no_image_package.into_iter() {
//             let images_package = goods_id_to_images_package
//                 .get(&sku.goods_id)
//                 .unwrap_or(&default_images_package);
//             skus.push(SKUModelDto::from_sku_and_images_package(
//                 sku,
//                 images_package.clone(),
//             ));
//         }
//
//         Ok(skus)
//     }
// }
