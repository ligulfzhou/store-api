#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;

pub use self::error::{ERPError, ERPResult};
use crate::config::database::DatabaseTrait;
use crate::config::{database, parameter};
use std::net::SocketAddr;
use std::sync::Arc;
// use tokio::net::TcpListener;

mod common;
mod config;
mod constants;
mod dto;
mod error;
mod excel;
mod handler;
mod middleware;
mod model;
mod repository;
mod response;
mod service;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    parameter::init();
    let port = parameter::get("PORT")
        .parse::<u16>()
        .expect("port should be number");

    let database = database::Database::init()
        .await
        .unwrap_or_else(|e| panic!("Database error: {}", e));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("=> Listen on {addr} \n");

    // axum::serve(
    //     TcpListener::bind(&addr).await?,
    //     handler::routes(Arc::new(database)),
    // )
    // .await
    // .expect("TODO: panic message");
    axum::Server::bind(&addr)
        .serve(handler::routes(Arc::new(database)))
        .await
        .unwrap();
}
