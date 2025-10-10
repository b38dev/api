pub mod error;
#[cfg(feature = "v1")]
pub mod v1;

pub use error::Result;

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

    let app = axum::Router::new();
    #[cfg(feature = "v1")]
    let app = app.nest_service("/v1", v1::routes().with_state(state.clone()));
    tracing::info!("Listening on http://{}", listen.local_addr()?);
    // println!("Listening on http://{}", listen.local_addr()?);
    axum::serve(listen, app).await?;
    Ok(())
}
