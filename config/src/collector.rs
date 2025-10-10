use rand::seq::IndexedRandom;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub onair: OnAir,
    pub user: User,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            onair: OnAir::default(),
            user: User::default(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OnAir {
    #[serde(default = "OnAir::default_mirror")]
    pub mirror: String,
}

impl OnAir {
    pub fn default_mirror() -> String {
        "https://github.com/bangumi-data/bangumi-data/raw/refs/heads/master/dist/data.json"
            .to_string()
    }
}

impl Default for OnAir {
    fn default() -> Self {
        Self {
            mirror: OnAir::default_mirror(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub origins: Vec<String>,
}

impl User {
    pub fn default_origins() -> Vec<String> {
        vec![
            "https://bgm.tv".to_string(),
            "https://chii.in".to_string(),
            "https://bangumi.tv".to_string(),
        ]
    }

    pub fn random_origin(&self) -> &String {
        let mut rng = rand::rng();
        self.origins
            .as_slice()
            .choose(&mut rng)
            .expect("No origins available")
    }
}
impl Default for User {
    fn default() -> Self {
        Self {
            origins: User::default_origins(),
        }
    }
}
