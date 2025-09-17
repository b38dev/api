use crate::states::{onair::DAILY_REFRESH_INTERVAL, AppState};
use std::sync::Arc;
use tracing::{error, info};

pub fn refresh(app_state: Arc<AppState>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(DAILY_REFRESH_INTERVAL);
        loop {
            interval.tick().await;
            info!("开始每日 on-air 数据刷新...");
            if let Err(e) = app_state.onair.write().await.refresh().await {
                error!("每日 on-air 数据刷新失败: {:?}", e);
            } else {
                info!("每日 on-air 数据刷新成功。");
            }
        }
    });
}
