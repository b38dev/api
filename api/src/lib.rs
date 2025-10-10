pub mod error;
#[cfg(feature = "v1")]
pub mod v1;

use axum::http::Method;
pub use error::Result;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::Level;

#[derive(Clone)]
pub struct AppState {}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn run() -> anyhow::Result<()> {
    let state = AppState::new();
    let listen = config::get().server.get_listen();
    let listen = tokio::net::TcpListener::bind(&listen).await?;

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);
    let trace_layer =
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().level(Level::INFO));
    let app = axum::Router::new();
    #[cfg(feature = "v1")]
    let app = app.nest_service("/v1", v1::routes().with_state(state.clone()));
    let app = app.layer(cors_layer).layer(trace_layer);
    tracing::info!("Listening on http://{}", listen.local_addr()?);
    axum::serve(listen, app).await?;
    Ok(())
}
