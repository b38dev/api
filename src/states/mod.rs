#[cfg(feature = "onair")]
pub mod onair;
#[cfg(feature = "user")]
pub mod user;
use crate::{config::AppConfig, error::AppError};
#[cfg(feature = "onair")]
use onair::OnAir;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    #[cfg(feature = "onair")]
    pub onair: Arc<OnAir>,
}

impl AppState {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self {
            config: config.clone(),
            #[cfg(feature = "onair")]
            onair: Arc::new(OnAir::new(&config.modules.onair)),
        }
    }

    pub async fn init(&self) -> Result<(), AppError> {
        #[cfg(feature = "onair")]
        self.onair.init().await?;
        Ok(())
    }

    pub fn get_config() {}
}
