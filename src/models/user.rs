use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct User {
    pub update: String,
    pub nid: u64,
    pub join: Option<String>,
    pub id: Option<String>,
    pub name_update: Option<String>,
    pub names: Option<HashSet<String>>,
}
