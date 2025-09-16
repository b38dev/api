pub mod error;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod state;

use axum::Router;
use routes::create_router;
use state::AppState;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::Level;

pub fn create_app() -> Router<AppState> {
    // 创建一个 tracing layer 用于记录请求日志
    let trace_layer =
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().level(Level::INFO));

    create_router().layer(trace_layer)
}
