use rand::seq::IndexedRandom;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub onair: OnAir,
    #[serde(default)]
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
pub struct UserFreshDuration {
    #[serde(default = "UserFreshDuration::default_active_duration")]
    pub active: i64,
    #[serde(default = "UserFreshDuration::default_abondon_duration")]
    pub abondon: i64,
    #[serde(default = "UserFreshDuration::default_dropped_duration")]
    pub dropped: i64,
    #[serde(default = "UserFreshDuration::default_banned_duration")]
    pub banned: i64,
}

impl UserFreshDuration {
    pub fn default_active_duration() -> i64 {
        1
    }

    pub fn default_abondon_duration() -> i64 {
        30
    }

    pub fn default_dropped_duration() -> i64 {
        365 * 100
    }

    pub fn default_banned_duration() -> i64 {
        365 * 100
    }
}

impl Default for UserFreshDuration {
    fn default() -> Self {
        Self {
            active: Self::default_active_duration(),
            abondon: Self::default_abondon_duration(),
            dropped: Self::default_dropped_duration(),
            banned: Self::default_banned_duration(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    #[serde(default = "User::default_origins")]
    pub origins: Vec<String>,
    #[serde(default)]
    pub fresh_duration: UserFreshDuration,
    #[serde(default = "User::default_active_month")]
    pub active_month: u32,
}

impl User {
    pub fn default_origins() -> Vec<String> {
        vec![
            "https://bgm.tv".to_string(),
            "https://chii.in".to_string(),
            "https://bangumi.tv".to_string(),
        ]
    }

    pub fn default_active_month() -> u32 {
        6
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
            fresh_duration: UserFreshDuration::default(),
            active_month: User::default_active_month(),
        }
    }
}
