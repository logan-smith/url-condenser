use axum::{
    routing::{get, post},
    Router,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, sync::RwLock};
use tower_http::trace::TraceLayer;

use crate::config::CONFIG;
use crate::handlers::health::get_health_endpoint;
use crate::handlers::url::{create_url_endpoint, get_url_endpoint};

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
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    let shared_state = SharedState::default();
    let routes = Router::new()
        .route("/health", get(get_health_endpoint))
        .route("/:short_url_code", get(get_url_endpoint))
        .route("/", post(create_url_endpoint));

    Router::new()
        .merge(routes)
        .with_state(Arc::clone(&shared_state))
        .layer(TraceLayer::new_for_http())
}

type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
pub struct AppState {
    db: HashMap<String, String>,
}
