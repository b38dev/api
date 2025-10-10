use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Fetcher {
    pub max_conn: Option<usize>,
    pub timeout_secs: Option<u64>,
    #[serde(default = "Fetcher::default_use_proxy")]
    pub use_proxy: bool,
    pub headers: Option<HashMap<String, String>>,
}

impl Fetcher {
    pub fn default_use_proxy() -> bool {
        false
    }
}

impl Default for Fetcher {
    fn default() -> Self {
        Self {
            max_conn: None,
            timeout_secs: None,
            use_proxy: Self::default_use_proxy(),
            headers: None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Clients {
    #[serde(default)]
    pub onair: Fetcher,
    #[serde(default)]
    pub bangumi: Fetcher,
}

impl Default for Clients {
    fn default() -> Self {
        Self {
            onair: Fetcher::default(),
            bangumi: Fetcher::default(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Proxy {
    pub host: Option<String>,
    pub port: Option<u16>,
}

impl Proxy {
    pub fn get_uri(&self) -> Option<String> {
        if self.host.is_none() || self.port.is_none() {
            return None;
        }
        Some(format!(
            "{}:{}",
            self.host.as_ref().unwrap(),
            self.port.unwrap()
        ))
    }
}

impl Default for Proxy {
    fn default() -> Self {
        Self {
            host: None,
            port: None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub proxy: Proxy,
    #[serde(default)]
    pub clients: Clients,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            proxy: Proxy::default(),
            clients: Clients::default(),
        }
    }
}
