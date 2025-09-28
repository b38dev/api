use crate::config::UserConfig;
use crate::models::onair::BangumiItem;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub items: HashMap<String, BangumiItem>,
    pub hash: String,
}

impl Data {
    fn get(&self, id: &str) -> Option<&BangumiItem> {
        self.items.get(id)
    }
}

#[derive(Debug)]
pub struct User {}

impl User {
    pub fn new(config: &UserConfig) -> Self {
        Self {}
    }
}
