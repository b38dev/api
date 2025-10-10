#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_filter")]
    pub filter: String,
}

impl Config {
    pub fn default_filter() -> String {
        "warn".into()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            filter: Self::default_filter(),
        }
    }
}
