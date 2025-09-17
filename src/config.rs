pub struct Config {
    pub listen: String,
    pub db_connection: String,
    pub onair_mirror: String,
    pub onair_cache_path: String,
    pub proxy: Option<String>,
}

pub fn get_config() -> Config {
    Config {
        listen: std::env::var("LISTEN").unwrap_or_else(|_| "127.0.0.1:3000".to_string()),
        db_connection: std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "mongodb://admin:password@127.0.0.1:27017/?authSource=admin".to_string()
        }),
        onair_mirror: std::env::var("ONAIR_MIRROR").unwrap_or_else(|_| {
            "https://cdn.jsdelivr.net/gh/bangumi-data/bangumi-data@latest/dist/data.json"
                .to_string()
        }),
        onair_cache_path: std::env::var("ONAIR_CACHE_PATH")
            .unwrap_or_else(|_| "cache/data.json".to_string()),
        proxy: std::env::var("PROXY").map_or(None, |s| Some(s)),
    }
}
