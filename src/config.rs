pub struct Config {
    pub listen: String,
    pub db_connection: String,
    pub onair_mirror: String,
    pub onair_cache_path: String,
}

pub fn get_config() -> Config {
    let listen = std::env::var("LISTEN").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let db_connection = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "mongodb://admin:password@127.0.0.1:27017/?authSource=admin".to_string()
    });
    let onair_mirror = std::env::var("ONAIR_MIRROR").unwrap_or_else(|_| {
        "https://cdn.jsdelivr.net/gh/bangumi-data/bangumi-data@latest/dist/data.json".to_string()
    });
    let onair_cache_path =
        std::env::var("ONAIR_CACHE_PATH").unwrap_or_else(|_| "cache/data.json".to_string());

    Config {
        listen,
        db_connection,
        onair_mirror,
        onair_cache_path,
    }
}
