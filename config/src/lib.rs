pub mod collector;
pub mod db;
pub mod fetcher;
pub mod scheduler;
pub mod server;
pub mod tracing;

use clap::Parser;
use figment::{
    Figment,
    providers::{Format, Json, Toml, Yaml},
};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to initialize config"));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: db::Config,
    #[serde(default)]
    pub server: server::Config,
    #[serde(default)]
    pub scheduler: scheduler::Config,
    #[serde(default)]
    pub tracing: tracing::Config,
    #[serde(default)]
    pub fetcher: fetcher::Config,
    #[serde(default)]
    pub collector: collector::Config,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let args = Args::parse();
        let config = Figment::new();
        let config = if args.config.ends_with(".toml") {
            config.merge(Toml::file(args.config))
        } else if args.config.ends_with(".yaml") || args.config.ends_with(".yml") {
            config.merge(Yaml::file(args.config))
        } else if args.config.ends_with(".json") {
            config.merge(Json::file(args.config))
        } else {
            return Err(anyhow::anyhow!(
                "Unsupported config file format: {}",
                args.config
            ));
        };
        let config = config.extract()?;
        Ok(config)
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // config file path, support YAML
    #[arg(short, long)]
    config: String,
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
