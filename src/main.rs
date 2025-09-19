pub mod config;
pub mod error;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod states;
pub mod tasks;

use std::sync::Arc;

use axum::http::Method;
use axum::Router;
use routes::create_router;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::Level;

use states::AppState;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let config = Arc::new(config::get_config());
    let state = Arc::new(AppState::new(config.clone()));

    state.init().await;
    tasks::start_tasks(state.clone());

    let app = create_app().with_state(state);

    let listener = TcpListener::bind(&config.listen).await.unwrap();
    info!("服务启动于 http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

pub fn create_app() -> Router<Arc<AppState>> {
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);
    let trace_layer =
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().level(Level::INFO));

    create_router().layer(cors_layer).layer(trace_layer)
}
