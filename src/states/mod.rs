pub mod onair;
use onair::OnAir;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub onair: Arc<RwLock<OnAir>>,
}

impl AppState {
    pub fn new(mirror: &str, cache_path: &str, proxy: Option<String>) -> Self {
        Self {
            onair: Arc::new(RwLock::new(OnAir::new(mirror, cache_path, proxy))),
        }
    }

    pub async fn init(&self) {
        self.onair.write().await.init().await.unwrap();
    }
}
