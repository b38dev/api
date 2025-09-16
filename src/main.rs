use axum_project_template::create_app;
use axum_project_template::state::{refresh_cache, AppState, DAILY_REFRESH_INTERVAL};
use tokio::net::TcpListener;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let app_state = AppState::new();

    if let Err(e) = refresh_cache(app_state.clone()).await {
        panic!("初始化数据失败: {:?}", e);
    }

    let state_for_task = app_state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(DAILY_REFRESH_INTERVAL);
        loop {
            interval.tick().await; // 等待下一个时间点
            if let Err(e) = refresh_cache(state_for_task.clone()).await {
                error!("每日缓存刷新失败: {:?}", e);
            }
        }
    });

    let app = create_app().with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("服务启动于 http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
