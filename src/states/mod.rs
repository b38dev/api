pub mod onair;
use crate::config::AppConfig;
use onair::OnAir;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub onair: Arc<RwLock<OnAir>>,
}

impl AppState {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self {
            config: config.clone(),
            onair: Arc::new(RwLock::new(OnAir::new(config.clone()))),
        }
    }

    pub async fn init(&self) {
        self.onair.write().await.init().await.unwrap();
    }
}
