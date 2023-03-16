use axum::{
    routing::{get, post},
    Router,
};
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, sync::RwLock};
use tower_http::trace::TraceLayer;

use crate::config::CONFIG;
use crate::database::*;
use crate::handlers::health::get_health_endpoint;
use crate::handlers::url::{create_alias_endpoint, get_alias_endpoint};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

pub mod config;
pub mod database;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod schema;
pub mod tests;
pub mod validate;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let config = CONFIG.clone();

    let addr: SocketAddr = config
        .server
        .parse()
        .expect("Unable to parse socket address");

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    // DB connection setup
    let config = CONFIG.clone();
    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool");
    let db_conn = pool;

    let routes = Router::new()
        .route("/health", get(get_health_endpoint))
        .route("/:short_url_code", get(get_alias_endpoint))
        .route("/", post(create_alias_endpoint));

    Router::new()
        .merge(routes)
        .with_state(db_conn)
        .layer(TraceLayer::new_for_http())
}

// type SharedState = Arc<RwLock<AppState>>;

// #[derive(Default)]
// pub struct AppState {
//     db: HashMap<String, String>,
// }
