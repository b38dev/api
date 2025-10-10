#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub onair: Scheduler,
    #[serde(default)]
    pub user: Scheduler,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            onair: Scheduler::default(),
            user: Scheduler::default(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Scheduler {
    #[serde(default = "Scheduler::default_cron")]
    pub cron: String,
    #[serde(default = "Scheduler::default_retry")]
    pub retry: u32,
}

impl Scheduler {
    pub fn default_cron() -> String {
        "0 0 0 * * *".to_string()
    }

    pub fn default_retry() -> u32 {
        1
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            cron: Self::default_cron(),
            retry: Self::default_retry(),
        }
    }
}
