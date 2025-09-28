use crate::{config::AppConfig, error::AppError, states::AppState};
use std::{sync::Arc, time::Duration};

#[derive(Clone, Debug)]
pub struct OnAirTask {
    pub name: String,
    pub retry: usize,
    pub interval: Duration,
}

impl super::Task for OnAirTask {
    async fn run(&self, state: Arc<AppState>) -> Result<(), AppError> {
        state.onair.write().await.refresh().await
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_retry(&self) -> usize {
        self.retry
    }

    fn get_interval(&self) -> Duration {
        self.interval.clone()
    }
}

pub fn task(config: Arc<AppConfig>) -> OnAirTask {
    OnAirTask {
        name: "OnAir 定时更新 bangumi-data".to_string(),
        interval: Duration::from_secs(config.onair.interval),
        retry: config.onair.retry,
    }
}
