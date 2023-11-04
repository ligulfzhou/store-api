use crate::config::database::Database;
use crate::middleware::auth::auth;
use crate::state::account_state::AccountState;
use crate::state::cate_state::CateState;
use axum::extract::DefaultBodyLimit;
use axum::http::{header, Method};
use axum::response::Response;
use axum::routing::IntoMakeService;
use axum::Router;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub mod routes_account;
pub mod routes_cates;
pub mod routes_customer;
pub mod routes_excel;
pub mod routes_items;
pub mod routes_login;
pub mod routes_static;
pub mod routes_upload;

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
            "https://erp.ligulfzhou.com".parse().unwrap(),
            "https://lien.ligulfzhou.com".parse().unwrap(),
            "http://127.0.0.1:3010".parse().unwrap(),
            "http://localhost:3010".parse().unwrap(),
            "http://127.0.0.1:3000".parse().unwrap(),
            "http://localhost:3000".parse().unwrap(),
            "https://egret-erp.vercel.app".parse().unwrap(),
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
            routes_cates::routes().with_state(CateState::new(&db)), // .layer(axum::middleware::from_fn_with_state(
                                                                    //     AccountState::new(&db),
                                                                    //     auth,
                                                                    // )),
        )
        // .merge(routes_upload::routes(app_state.clone()))
        // .merge(routes_account::routes(app_state.clone()))
        // .merge(routes_customer::routes(app_state.clone()))
        // .merge(routes_excel::routes(app_state.clone()))
        // .merge(routes_login::routes(app_state.clone()))
        // .merge(routes_items::routes(app_state.clone()))
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
    // tokio::time::sleep(std::time::Duration::new(0, 300)).await;
    res
}
