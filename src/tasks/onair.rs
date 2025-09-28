use crate::{config::OnAirConfig, error::AppError, states::AppState};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct OnAirTask {
    pub name: String,
    pub retry: usize,
    pub cron: String,
}

impl super::Task for OnAirTask {
    async fn run(&self, state: Arc<AppState>) -> Result<(), AppError> {
        state.onair.refresh().await
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_retry(&self) -> usize {
        self.retry
    }

    fn get_cron(&self) -> String {
        self.cron.clone()
    }
}

pub fn task(config: &OnAirConfig) -> OnAirTask {
    OnAirTask {
        name: "OnAir 定时更新 bangumi-data".to_string(),
        cron: config.cron.clone(),
        retry: config.retry,
    }
}
