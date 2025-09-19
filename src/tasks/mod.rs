pub mod onair;
use crate::{error::AppError, states::AppState};
use std::{future::Future, sync::Arc, time::Duration};
use tracing::{error, info, warn};

pub trait Task {
    fn run(&self, state: Arc<AppState>) -> impl Future<Output = Result<(), AppError>> + Send;
    fn get_name(&self) -> String;
    fn get_retry(&self) -> usize;
    fn get_interval(&self) -> Duration;
}

pub fn task_spawn(state: Arc<AppState>, task: impl Task + Send + 'static) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(task.get_interval());
        let times = task.get_retry();
        let name = task.get_name();
        loop {
            interval.tick().await;
            info!("[TASK][start] {name}");

            for turn in 1..times {
                info!("[TASK][running][{turn}/{times}] {name}");
                if let Err(e) = task.run(state.clone()).await {
                    warn!("[TASK][failed][{turn}/{times}] {name} {:?}", e);
                } else {
                    info!("[TASK][done] {name}");
                    return;
                }
            }

            error!("[TASK][failed] {name}");
        }
    });
}

pub fn start_tasks(state: Arc<AppState>) {
    task_spawn(state.clone(), onair::task(state.config.clone()));
}
