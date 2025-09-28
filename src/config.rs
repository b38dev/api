use clap::Parser;
use figment::{
    Figment,
    providers::{Env, Format, Yaml},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub listen: String,
    pub modules: Modules,
    pub proxy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modules {
    #[cfg(feature = "onair")]
    pub onair: OnAirConfig,
    #[cfg(feature = "user")]
    pub user: UserConfig,
}

#[cfg(feature = "onair")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnAirConfig {
    pub use_proxy: Option<bool>,
    pub proxy: Option<String>,
    pub mirror: String,
    // "https://cdn.jsdelivr.net/gh/bangumi-data/bangumi-data@latest/dist/data.json"
    // "https://github.com/bangumi-data/bangumi-data/raw/refs/heads/master/dist/data.json"
    pub cache_path: String,
    // "cache/data.json"
    pub retry: usize,
    pub cron: String,
}

#[cfg(feature = "user")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub use_proxy: Option<bool>,
    pub proxy: Option<String>,
    // https://bgm.tv/user/vickscarlet/timeline\?type\=say\&ajax\=1
    pub max_conn: usize,
    pub domains: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // config file path, support YAML
    #[arg(short, long)]
    config: String,
}

pub fn get_config() -> AppConfig {
    let args = Args::parse();
    let mut config: AppConfig = Figment::new()
        .merge(Yaml::file(args.config))
        .merge(Env::raw().only(&["PROXY", "LISTEN"]))
        .extract()
        .unwrap();
    #[cfg(feature = "onair")]
    if let Some(true) = config.modules.onair.use_proxy {
        config.modules.onair.proxy = config.proxy.clone()
    }

    #[cfg(feature = "user")]
    if let Some(true) = config.modules.user.use_proxy {
        config.modules.user.proxy = config.proxy.clone()
    }

    config
}
