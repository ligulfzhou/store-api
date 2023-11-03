#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;

use crate::config::database::DatabaseTrait;
use crate::config::{database, parameter};
use std::net::SocketAddr;
use std::sync::Arc;

mod common;
mod config;
mod constants;
mod dto;
mod error;
mod excel;
mod handler;
mod middleware;
mod model;
mod response;
mod service;
mod state;
mod repository;

pub use self::error::{ERPError, ERPResult};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = parameter::get("PORT")
        .parse::<u16>()
        .expect("port should be number");

    let database = database::Database::init()
        .await
        .unwrap_or_else(|e| panic!("Database error: {}", e.to_string()));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("=> Listen on {addr} \n");

    axum::Server::bind(&addr)
        .serve(handler::routes(Arc::new(database)))
        .await
        .unwrap();
}
