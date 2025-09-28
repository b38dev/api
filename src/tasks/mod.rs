#[cfg(feature = "onair")]
pub mod onair;
use crate::{error::AppError, states::AppState};
use std::{future::Future, sync::Arc};
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info, warn};

pub trait Task {
    fn run(&self, state: Arc<AppState>) -> impl Future<Output = Result<(), AppError>> + Send;
    fn get_name(&self) -> String;
    fn get_retry(&self) -> usize;
    fn get_cron(&self) -> String;
}

fn create_job(
    state: Arc<AppState>,
    task: Arc<impl Task + Sync + Send + 'static>,
) -> Result<Job, AppError> {
    Ok(Job::new_async(task.get_cron(), move |uuid, mut l| {
        let name = task.get_name();
        let times = task.get_retry();
        let task = task.clone();
        let state = state.clone();
        Box::pin(async move {
            let _ = l.next_tick_for_job(uuid).await;

            info!("[TASK][start] {name}");

            for turn in 1..times + 1 {
                info!("[TASK][running][{turn}/{times}] {name}");
                if let Err(e) = task.run(state.clone()).await {
                    warn!("[TASK][failed][{turn}/{times}] {name} {:?}", e);
                } else {
                    info!("[TASK][done] {name}");
                    return;
                }
            }

            error!("[TASK][failed] {name}");
        })
    })?)
}

pub async fn start_scheduler(state: Arc<AppState>) -> Result<(), AppError> {
    let sched = JobScheduler::new().await?;
    #[cfg(feature = "onair")]
    let onair_task = Arc::new(onair::task(&state.config.modules.onair));
    #[cfg(feature = "onair")]
    sched.add(create_job(state.clone(), onair_task)?).await?;
    sched.start().await?;
    Ok(())
}

pub fn start_tasks(state: Arc<AppState>) {
    tokio::spawn(start_scheduler(state));
}
