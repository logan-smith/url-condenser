use axum::{
    routing::{get, post},
    Router,
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use crate::config::CONFIG;
use crate::handlers::health::get_health_endpoint;
use crate::handlers::url::{create_alias_endpoint, get_alias_endpoint};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

pub mod config;
pub mod errors;
pub mod handlers;
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
        .serve(app().await.into_make_service())
        .await
        .unwrap();
}

async fn app() -> Router {
    // DB connection setup
    let config = CONFIG.clone();
    let db_conn = Database::connect(config.database_url)
        .await
        .expect("Database connection failed");

    // Run migrations
    Migrator::up(&db_conn, None).await.unwrap();

    let state = AppState { db_conn };

    let routes = Router::new()
        .route("/health", get(get_health_endpoint))
        .route("/:short_url_code", get(get_alias_endpoint))
        .route("/", post(create_alias_endpoint));

    Router::new()
        .merge(routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

#[derive(Clone)]
pub struct AppState {
    db_conn: DatabaseConnection,
}
