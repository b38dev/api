pub mod error;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod states;
pub mod tasks;

use std::sync::Arc;

use axum::Router;
use routes::create_router;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::Level;

use states::AppState;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let app_state = Arc::new(AppState::new(
        "https://cdn.jsdelivr.net/gh/bangumi-data/bangumi-data@latest/dist/data.json",
        "cache/data.json",
    ));

    app_state.init().await;
    tasks::start_tasks(app_state.clone());

    let app = create_app().with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("服务启动于 http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

pub fn create_app() -> Router<Arc<AppState>> {
    let trace_layer =
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().level(Level::INFO));

    create_router().layer(trace_layer)
}
