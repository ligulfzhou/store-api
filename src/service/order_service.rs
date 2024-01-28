use crate::config::database::{Database, DatabaseTrait};
use crate::constants::DEFAULT_PAGE_SIZE;
use crate::dto::dto_orders::{
    CreateOrderParams, OrderDto, OrderInListDto, OrderItemDto, OrderItemsParams, QueryParams,
};
use crate::model::order::{ImportedOrderItemModel, OrderItemModel, OrderModel};
use crate::ERPResult;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{query, FromRow, Postgres, QueryBuilder};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct OrderService {
    pub db: Arc<Database>,
}

#[async_trait]
pub trait OrderServiceTrait {
    fn new(db: &Arc<Database>) -> Self;
    async fn create_order(&self, account_id: i32, params: &CreateOrderParams) -> ERPResult<i32>;
    async fn add_order(&self, order: &OrderModel) -> ERPResult<i32>;
    async fn insert_order_items(
        &self,
        items: &[OrderItemsParams],
        order_id: i32,
    ) -> ERPResult<Vec<OrderItemModel>>;
    async fn insert_just_order_items(
        &self,
        items: &[OrderItemModel],
    ) -> ERPResult<Vec<OrderItemModel>>;
    async fn insert_just_imported_order_items(
        &self,
        items: &[ImportedOrderItemModel],
    ) -> ERPResult<Vec<ImportedOrderItemModel>>;
    async fn get_order_list(&self, params: &QueryParams) -> ERPResult<Vec<OrderInListDto>>;
    async fn get_count_order_list(&self, params: &QueryParams) -> ERPResult<i32>;
    async fn get_imported_order_list(&self, params: &QueryParams)
        -> ERPResult<Vec<OrderInListDto>>;
    async fn get_count_imported_order_list(&self, params: &QueryParams) -> ERPResult<i32>;
    async fn get_order(&self, order_id: i32) -> ERPResult<OrderDto>;
    async fn get_order_items(&self, order_id: i32) -> ERPResult<Vec<OrderItemDto>>;
    async fn get_imported_order_items(
        &self,
        order_id: i32,
    ) -> ERPResult<Vec<ImportedOrderItemModel>>;
    async fn delete_order(&self, order_id: i32) -> ERPResult<()>;
    async fn delete_import_order(&self, order_id: i32) -> ERPResult<()>;
}

#[derive(Debug, Serialize, FromRow)]
pub struct TmpOrderInListDto {
    pub id: i32,
    pub account_id: i32,
    pub account: String,
    pub customer_id: i32,
    pub customer: String,
    pub order_date: NaiveDate,
    pub delivery_date: NaiveDate,
    pub create_time: DateTime<Utc>,
}

#[async_trait]
impl OrderServiceTrait for OrderService {
    fn new(db: &Arc<Database>) -> Self {
        Self { db: Arc::clone(db) }
    }

    async fn create_order(&self, account_id: i32, params: &CreateOrderParams) -> ERPResult<i32> {
        let order = sqlx::query!(
            "insert into orders(account_id, customer_id) values ($1, $2) returning *",
            account_id,
            params.customer_id
        )
        .fetch_one(self.db.get_pool())
        .await?;

        let order_items = self.insert_order_items(&params.items, order.id).await?;

        Ok(order.id)
    }

    async fn add_order(&self, order: &OrderModel) -> ERPResult<i32> {
        let order = sqlx::query!(
            r#"
            insert into orders(account_id, customer_id, order_date, tp, delivery_date)
            values ($1, $2, $3, $4, $5) 
            returning *;
            "#,
            order.account_id,
            order.customer_id,
            order.order_date,
            order.tp,
            order.delivery_date
        )
        .fetch_one(self.db.get_pool())
        .await?;

        Ok(order.id)
    }

    async fn insert_order_items(
        &self,
        items: &[OrderItemsParams],
        order_id: i32,
    ) -> ERPResult<Vec<OrderItemModel>> {
        let item_ids = items.iter().map(|item| item.item_id).collect::<Vec<_>>();
        let item_id_to_origin_price =
            sqlx::query!("select id, price from items where id = any($1)", &item_ids)
                .fetch_all(self.db.get_pool())
                .await?
                .into_iter()
                .map(|item| (item.id, item.price))
                .collect::<HashMap<_, _>>();

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "insert into order_items (order_id, item_id, count, price, origin_price, discount, total_price) ",
        );

        query_builder.push_values(items, |mut b, item| {
            let origin_price = item_id_to_origin_price.get(&item.item_id).unwrap_or(&0);
            // let price = origin_price * item.discount / 100;
            // let total_price = item.count * origin_price * item.discount / 100;
            let total_price = item.count * item.discount_price / 10;
            b.push_bind(order_id)
                .push_bind(item.item_id)
                .push_bind(item.count)
                .push_bind(item.discount_price)
                .push_bind(*origin_price)
                .push_bind(item.discount)
                .push_bind(total_price);
        });
        query_builder.push(" returning *;");

        let res = query_builder
            .build_query_as::<OrderItemModel>()
            .fetch_all(self.db.get_pool())
            .await?;

        Ok(res)
    }

    async fn insert_just_order_items(
        &self,
        items: &[OrderItemModel],
    ) -> ERPResult<Vec<OrderItemModel>> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "insert into order_items (order_id, item_id, count, price, origin_price, discount, total_price) ",
        );

        query_builder.push_values(items, |mut b, item| {
            b.push_bind(item.order_id)
                .push_bind(item.item_id)
                .push_bind(item.count)
                .push_bind(item.price)
                .push_bind(item.origin_price)
                .push_bind(item.discount)
                .push_bind(item.total_price);
        });
        query_builder.push(" returning *;");

        let res = query_builder
            .build_query_as::<OrderItemModel>()
            .fetch_all(self.db.get_pool())
            .await?;

        Ok(res)
    }

    async fn insert_just_imported_order_items(
        &self,
        items: &[ImportedOrderItemModel],
    ) -> ERPResult<Vec<ImportedOrderItemModel>> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "insert into import_order_items (order_id, number, images, size, name, color, count, unit, price, total_price, create_time) ",
        );

        query_builder.push_values(items, |mut b, item| {
            b.push_bind(item.order_id)
                .push_bind(item.number.clone())
                .push_bind(item.images.clone())
                .push_bind(item.size.clone())
                .push_bind(item.name.clone())
                .push_bind(item.color.clone())
                .push_bind(item.count)
                .push_bind(item.unit.clone())
                .push_bind(item.price)
                .push_bind(item.total_price)
                .push_bind(item.create_time);
        });
        query_builder.push(" returning *;");

        let res = query_builder
            .build_query_as::<ImportedOrderItemModel>()
            .fetch_all(self.db.get_pool())
            .await?;

        Ok(res)
    }

    async fn get_order_list(&self, params: &QueryParams) -> ERPResult<Vec<OrderInListDto>> {
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            select o.*, a.name as account, c.name as customer from orders o, accounts a, customers c 
            where o.customer_id = c.id and o.account_id = a.id 
                and o.tp = 0
            "#,
        );

        if params.account_id != 0 {
            sql.push("and o.account_id = ").push_bind(params.account_id);
        }

        if params.customer_id != 0 {
            sql.push("and o.customer_id = ")
                .push_bind(params.customer_id);
        }

        if !params.create_time_st.is_empty() && !params.create_time_ed.is_empty() {
            sql.push(" and o.create_time >= ")
                .push_bind(&params.create_time_st)
                .push(" and o.create_time <= ")
                .push_bind(&params.create_time_ed);
        }
        //     let field = param.sorter_field.as_deref().unwrap_or("id");
        //     let order = param.sorter_order.as_deref().unwrap_or("desc");
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        let offset = (page - 1) * page_size;

        sql.push(format!(
            " order by id desc limit {} offset {}",
            page_size, offset
        ));

        let orders = sql
            .build_query_as::<TmpOrderInListDto>()
            .fetch_all(self.db.get_pool())
            .await?;

        let order_ids = orders.iter().map(|item| item.id).collect::<Vec<_>>();
        let order_id_to_count_sum = sqlx::query!(
            r#"
            select order_id, sum(count) as count, sum(total_price) as total
            from order_items 
            where order_id = any($1) 
            group by order_id;
            "#,
            &order_ids
        )
        .fetch_all(self.db.get_pool())
        .await?
        .into_iter()
        .map(|item| {
            (
                item.order_id,
                (
                    item.count.unwrap_or(0) as i32,
                    item.total.unwrap_or(0) as i32,
                ),
            )
        })
        .collect::<HashMap<_, _>>();

        let mut order_id_to_images: HashMap<i32, Vec<String>> = HashMap::new();
        sqlx::query!(
            r#"
            select oi.order_id, i.images
            from order_items oi, items i 
            where oi.item_id = i.id and oi.order_id = any($1);
            "#,
            &order_ids
        )
        .fetch_all(self.db.get_pool())
        .await?
        .into_iter()
        .for_each(|item| {
            print!("item: {:?}", item);
            if !item.images.is_empty() {
                let cur_images = order_id_to_images.entry(item.order_id).or_insert(vec![]);
                if cur_images.len() >= 3 {
                    return;
                }
                cur_images.push(item.images[0].clone());
            }
        });
        print!("order_id_to_images: {:?}", order_id_to_images);

        let empty_str_arr: Vec<String> = vec![];
        let order_list = orders
            .into_iter()
            .map(|order| {
                let count_sum = order_id_to_count_sum.get(&order.id).unwrap_or(&(0, 0));
                let images = order_id_to_images.get(&order.id).unwrap_or(&empty_str_arr);
                OrderInListDto {
                    id: order.id,
                    account_id: order.account_id,
                    account: order.account,
                    customer_id: order.customer_id,
                    customer: order.customer,
                    item_images: images.clone(),
                    create_time: order.create_time,
                    count: count_sum.0,
                    total: count_sum.1,
                }
            })
            .collect::<Vec<_>>();

        Ok(order_list)
    }

    async fn get_count_order_list(&self, params: &QueryParams) -> ERPResult<i32> {
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            select count(1)
            from orders o, accounts a, customers c 
            where o.customer_id = c.id and o.account_id = a.id
                and o.tp = 0
            "#,
        );

        if params.account_id != 0 {
            sql.push("and o.account_id = ").push_bind(params.account_id);
        }

        if params.customer_id != 0 {
            sql.push("and o.customer_id = ")
                .push_bind(params.customer_id);
        }

        if !params.create_time_st.is_empty() && !params.create_time_ed.is_empty() {
            sql.push(" and o.create_time >= ")
                .push_bind(&params.create_time_st)
                .push(" and o.create_time <= ")
                .push_bind(&params.create_time_ed);
        }

        let count = sql
            .build_query_as::<(i64,)>()
            .fetch_one(self.db.get_pool())
            .await?
            .0 as i32;

        Ok(count)
    }

    async fn get_imported_order_list(
        &self,
        params: &QueryParams,
    ) -> ERPResult<Vec<OrderInListDto>> {
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            select o.*, a.name as account, c.name as customer from orders o, accounts a, customers c 
            where o.customer_id = c.id and o.account_id = a.id 
                and o.tp = 1
            "#,
        );

        if params.account_id != 0 {
            sql.push("and o.account_id = ").push_bind(params.account_id);
        }

        if params.customer_id != 0 {
            sql.push("and o.customer_id = ")
                .push_bind(params.customer_id);
        }

        if !params.create_time_st.is_empty() && !params.create_time_ed.is_empty() {
            sql.push(" and o.create_time >= ")
                .push_bind(&params.create_time_st)
                .push(" and o.create_time <= ")
                .push_bind(&params.create_time_ed);
        }
        //     let field = param.sorter_field.as_deref().unwrap_or("id");
        //     let order = param.sorter_order.as_deref().unwrap_or("desc");
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        let offset = (page - 1) * page_size;

        sql.push(format!(
            " order by id desc limit {} offset {}",
            page_size, offset
        ));

        let orders = sql
            .build_query_as::<TmpOrderInListDto>()
            .fetch_all(self.db.get_pool())
            .await?;

        let order_ids = orders.iter().map(|item| item.id).collect::<Vec<_>>();
        let order_id_to_count_sum = sqlx::query!(
            r#"
            select order_id, sum(count) as count, sum(total_price) as total
            from import_order_items 
            where order_id = any($1) 
            group by order_id;
            "#,
            &order_ids
        )
        .fetch_all(self.db.get_pool())
        .await?
        .into_iter()
        .map(|item| {
            (
                item.order_id,
                (
                    item.count.unwrap_or(0) as i32,
                    item.total.unwrap_or(0) as i32,
                ),
            )
        })
        .collect::<HashMap<_, _>>();
        tracing::info!("order_id_to_count_sum: {:?}", order_id_to_count_sum);

        let mut order_id_to_images: HashMap<i32, Vec<String>> = HashMap::new();
        sqlx::query!(
            r#"
            select ioi.order_id, ioi.images 
            from import_order_items ioi
            where ioi.order_id = any($1);
            "#,
            &order_ids
        )
        .fetch_all(self.db.get_pool())
        .await?
        .iter()
        .for_each(|item| {
            print!("item: {:?}", item);
            if !item.images.is_empty() {
                let cur_images = order_id_to_images.entry(item.order_id).or_insert(vec![]);
                if cur_images.len() >= 3 {
                    return;
                }
                cur_images.push(item.images[0].clone());
            }
        });

        let empty_str_arr: Vec<String> = vec![];
        let order_list = orders
            .into_iter()
            .map(|order| {
                let count_sum = order_id_to_count_sum.get(&order.id).unwrap_or(&(0, 0));
                let images = order_id_to_images.get(&order.id).unwrap_or(&empty_str_arr);
                OrderInListDto {
                    id: order.id,
                    account_id: order.account_id,
                    account: order.account,
                    customer_id: order.customer_id,
                    customer: order.customer,
                    item_images: images.clone(),
                    create_time: order.create_time,
                    count: count_sum.0,
                    total: count_sum.1,
                }
            })
            .collect::<Vec<_>>();

        Ok(order_list)
    }

    async fn get_count_imported_order_list(&self, params: &QueryParams) -> ERPResult<i32> {
        let mut sql: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            select count(1)
            from orders o, accounts a, customers c 
            where o.customer_id = c.id and o.account_id = a.id
                and o.tp = 1
            "#,
        );

        if params.account_id != 0 {
            sql.push("and o.account_id = ").push_bind(params.account_id);
        }

        if params.customer_id != 0 {
            sql.push("and o.customer_id = ")
                .push_bind(params.customer_id);
        }

        if !params.create_time_st.is_empty() && !params.create_time_ed.is_empty() {
            sql.push(" and o.create_time >= ")
                .push_bind(&params.create_time_st)
                .push(" and o.create_time <= ")
                .push_bind(&params.create_time_ed);
        }

        let count = sql
            .build_query_as::<(i64,)>()
            .fetch_one(self.db.get_pool())
            .await?
            .0 as i32;

        Ok(count)
    }

    async fn get_order(&self, order_id: i32) -> ERPResult<OrderDto> {
        Ok(sqlx::query_as!(
            OrderDto,
            r#"
            select 
                o.*, 
                a.name as account, 
                c.name as customer
            from orders o, accounts a, customers c 
            where o.account_id = a.id and o.customer_id = c.id 
                and o.id = $1;
            "#,
            order_id,
        )
        .fetch_one(self.db.get_pool())
        .await?)
    }

    async fn get_order_items(&self, order_id: i32) -> ERPResult<Vec<OrderItemDto>> {
        Ok(sqlx::query_as!(
            OrderItemDto,
            r#"
            select 
                oi.*,
                i.images, i.size, i.number, i.name, i.color
            from order_items oi, items i
            where oi.item_id = i.id
                and oi.order_id=$1
            "#,
            order_id
        )
        .fetch_all(self.db.get_pool())
        .await?)
    }

    async fn get_imported_order_items(
        &self,
        order_id: i32,
    ) -> ERPResult<Vec<ImportedOrderItemModel>> {
        Ok(sqlx::query_as!(
            ImportedOrderItemModel,
            "select * from import_order_items where order_id = $1 order by id",
            order_id
        )
        .fetch_all(self.db.get_pool())
        .await?)
    }

    async fn delete_order(&self, order_id: i32) -> ERPResult<()> {
        sqlx::query!("delete from orders where id = $1", order_id)
            .execute(self.db.get_pool())
            .await?;
        sqlx::query!("delete from order_items where order_id = $1", order_id)
            .execute(self.db.get_pool())
            .await?;
        let res = sqlx::query!(
            "select id from item_inout_bucket where order_id=$1",
            order_id
        )
        .fetch_optional(self.db.get_pool())
        .await?;
        if res.is_some() {
            let bucket_id = res.unwrap().id;

            sqlx::query!("delete from item_inout_bucket where id = $1", bucket_id)
                .execute(self.db.get_pool())
                .await?;
            sqlx::query!("delete from item_inout where bucket_id = $1", bucket_id)
                .execute(self.db.get_pool())
                .await?;
        }

        Ok(())
    }

    async fn delete_import_order(&self, order_id: i32) -> ERPResult<()> {
        sqlx::query!("delete from orders where id = $1", order_id)
            .execute(self.db.get_pool())
            .await?;
        sqlx::query!(
            "delete from import_order_items where order_id = $1",
            order_id
        )
        .execute(self.db.get_pool())
        .await?;
        Ok(())
    }
}
