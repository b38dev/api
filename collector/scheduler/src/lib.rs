pub mod onair;
pub mod user;

use futures::future::join_all;
use std::{pin::Pin, sync::Arc};

pub trait Task: Send + Sync {
    fn get_cron(&self) -> &str;
    fn get_name(&self) -> &str;
    fn get_retry(&self) -> u32;
    fn get_run_now(&self) -> bool;
    fn run(&self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>;
}

pub async fn add_job(
    scheduler: &tokio_cron_scheduler::JobScheduler,
    task: Arc<dyn Task>,
) -> anyhow::Result<tokio_cron_scheduler::job::JobId> {
    let cron = task.get_cron();
    let task_for_job = task.clone();
    let job = tokio_cron_scheduler::Job::new_async(cron, move |uuid, mut l| {
        let task_for_job = task_for_job.clone();
        Box::pin(async move {
            let _ = l.next_tick_for_job(uuid).await;
            run_task_retry(task_for_job.as_ref())
                .await
                .expect("Job failed");
        })
    })?;
    let id = scheduler.add(job).await?;
    if task.get_run_now() {
        let task = task.clone();
        tokio::spawn(async move {
            run_task_retry(task.as_ref()).await.expect("Job failed");
        });
    }
    Ok(id)
}

pub async fn run_task_retry(task: &dyn Task) -> anyhow::Result<()> {
    let name = task.get_name().to_string();
    let times = task.get_retry();
    tracing::info!("[Scheduler][start] {name}");
    for turn in 1..=times {
        tracing::info!("[Scheduler][running][{turn}/{times}] {name}");
        if let Err(e) = task.run().await {
            tracing::warn!("[Scheduler][failed][{turn}/{times}] {name} {:?}", e);
        } else {
            tracing::info!("[Scheduler][done] {name}");
            return Ok(());
        }
    }
    tracing::error!("[Scheduler][failed] {name}");
    Err(anyhow::anyhow!("Task {name} failed"))
}

pub async fn run() -> anyhow::Result<()> {
    tracing::info!("Scheduler started");
    let scheduler = tokio_cron_scheduler::JobScheduler::new().await?;
    let jobs = vec![onair::Task::new()]
        .into_iter()
        .map(|task| add_job(&scheduler, Arc::new(task)))
        .collect::<Vec<_>>();
    join_all(jobs).await;
    scheduler.start().await?;
    Ok(())
}
