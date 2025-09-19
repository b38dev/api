use clap::Parser;
use figment::{
    providers::{Env, Format, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub listen: String,
    pub onair: OnAirConfig,
    pub proxy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnAirConfig {
    pub mirror: String,
    // "https://cdn.jsdelivr.net/gh/bangumi-data/bangumi-data@latest/dist/data.json"
    // "https://github.com/bangumi-data/bangumi-data/raw/refs/heads/master/dist/data.json"
    pub cache_path: String,
    // "cache/data.json"
    pub retry: usize,
    pub interval: u64,
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
    let config: AppConfig = Figment::new()
        .merge(Yaml::file(args.config))
        .merge(Env::raw().only(&["PROXY", "LISTEN"]))
        .extract()
        .unwrap();
    config
}
