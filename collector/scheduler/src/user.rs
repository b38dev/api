use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct Task;

impl Task {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::Task for Task {
    fn get_cron(&self) -> &str {
        &config::get().scheduler.user.cron
    }

    fn get_name(&self) -> &str {
        "User Refresh"
    }

    fn get_retry(&self) -> u32 {
        config::get().scheduler.user.retry
    }

    fn get_run_now(&self) -> bool {
        true
    }

    fn run(&self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
        Box::pin(async move { Ok(()) })
    }
}
