#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub uri: String,
}

impl Config {
    pub fn get_uri(&self) -> &str {
        &self.uri
    }
}
