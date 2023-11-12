use crate::config::database::Database;
use crate::middleware::auth::auth;
use crate::state::account_state::AccountState;
use crate::state::cate_state::CateState;
use crate::state::customer_state::CustomerState;
use crate::state::item_state::ItemState;
use crate::state::settings_state::SettingsState;
use axum::extract::DefaultBodyLimit;
use axum::http::{header, Method};
use axum::response::Response;
use axum::routing::IntoMakeService;
use axum::Router;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod routes_account;
mod routes_cates;
mod routes_customer;
mod routes_excel;
mod routes_items;
mod routes_login;
mod routes_settings;
mod routes_static;
mod routes_upload;

pub trait ListParamToSQLTrait {
    fn to_pagination_sql(&self) -> String;
    fn to_count_sql(&self) -> String;
}

pub trait CreateOrUpdateParamToSQLTrait {
    fn to_sql(&self) -> String;
}

pub fn routes(db: Arc<Database>) -> IntoMakeService<Router> {
    let cors = CorsLayer::new()
        .allow_origin([
            "https://store-web-five.vercel.app".parse().unwrap(),
            "https://store.ligulfzhou.com".parse().unwrap(),
            "http://127.0.0.1:3010".parse().unwrap(),
            "http://localhost:3010".parse().unwrap(),
        ])
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_headers(vec![
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ORIGIN,
            header::AUTHORIZATION,
            header::HeaderName::from_lowercase(b"x-requested-with").unwrap(),
        ]);

    let routes_all = Router::new()
        .merge(
            routes_cates::routes()
                .with_state(CateState::new(&db))
                .layer(axum::middleware::from_fn_with_state(
                    AccountState::new(&db),
                    auth,
                )),
        )
        .merge(routes_customer::routes().with_state(CustomerState::new(&db)))
        .merge(
            routes_account::routes()
                .with_state(AccountState::new(&db))
                .layer(axum::middleware::from_fn_with_state(
                    AccountState::new(&db),
                    auth,
                )),
        )
        .merge(routes_items::routes().with_state(ItemState::new(&db)))
        .merge(routes_upload::routes())
        .merge(routes_settings::routes().with_state(SettingsState::new(&db)))
        .merge(routes_excel::routes().with_state(ItemState::new(&db)))
        .merge(routes_login::routes().with_state(AccountState::new(&db)))
        // todo: for test
        .layer(axum::middleware::map_response(main_response_mapper))
        .fallback_service(routes_static::routes())
        .layer(DefaultBodyLimit::max(usize::MAX))
        .layer(cors);

    routes_all.into_make_service()
}
async fn main_response_mapper(res: Response) -> Response {
    tracing::info!("->> {:<12} - main_response_mapper", "res_mapper");
    tracing::info!("{:?}", res.headers());
    // tokio::time::sleep(std::time::Duration::new(2, 0)).await;
    res
}
