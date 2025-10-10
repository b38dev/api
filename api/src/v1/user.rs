use crate::AppState;

use axum::{Json, Router, extract::Query, routing::get};
use model::prelude::{NameHistory, Uid, User, UserState};

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
    pub state: UserState,
    pub nid: Option<i32>,
    pub sid: Option<String>,
    pub name_history: Option<NameHistory>,
}

impl From<User> for Data {
    fn from(user: User) -> Self {
        Self {
            state: user.state,
            nid: user.nid,
            sid: user.sid,
            name_history: user.extra.name_history,
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
