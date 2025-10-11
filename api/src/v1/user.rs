use crate::AppState;

use axum::{Json, Router, extract::Query, routing::get};
use model::prelude::{Collections, NameHistory, Uid, User, UserState};

pub fn routes() -> Router<AppState> {
    Router::new().route("/name-history", get(query_name_history_by_uid))
}

#[derive(serde::Serialize)]
pub struct NameHistoryResponse {
    pub data: Data,
}

impl From<User> for NameHistoryResponse {
    fn from(data: User) -> Self {
        Self { data: data.into() }
    }
}

#[derive(serde::Serialize)]
pub struct Data {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    pub state: UserState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active: Option<chrono::DateTime<chrono::Utc>>,
    pub update_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_history: Option<NameHistory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Collections>,
}

impl From<User> for Data {
    fn from(user: User) -> Self {
        Self {
            name: user.name,
            nid: user.nid,
            sid: user.sid,
            state: user.state,
            join_time: user.join_time,
            last_active: user.last_active,
            update_at: user.update_at,
            name_history: user.extra.name_history,
            collections: user.extra.collections,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct NameHistoryQuery {
    #[serde(deserialize_with = "NameHistoryQuery::deserialize_uid")]
    uid: Uid,
}

impl NameHistoryQuery {
    fn deserialize_uid<'de, D>(deserializer: D) -> Result<Uid, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Ok(Uid::from_str(&s))
    }
}

#[axum::debug_handler]
pub async fn query_name_history_by_uid(
    Query(NameHistoryQuery { uid }): Query<NameHistoryQuery>,
) -> crate::Result<Json<NameHistoryResponse>> {
    tracing::debug!("Query name history for uid: {}", uid.to_string());
    let user = collector::user::query_user(uid).await?;
    return Ok(Json(user.into()));
}
