#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_host")]
    pub host: String,
    #[serde(default = "Config::default_port")]
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: Self::default_host(),
            port: Self::default_port(),
        }
    }
}

impl Config {
    pub fn default_host() -> String {
        "127.0.0.1".into()
    }

    pub fn default_port() -> u16 {
        3000
    }

    pub fn get_listen(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
