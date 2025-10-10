use crate::AppState;

use axum::{Json, Router, extract::Query, routing::get};
use model::prelude::{Names, Uid, User};

pub fn routes() -> Router<AppState> {
    Router::new().route("/name-history", get(query_name_history_by_uid))
}

#[derive(serde::Serialize)]
pub struct NameHistoryResponse {
    pub names: Names,
}

impl NameHistoryResponse {
    pub fn empty() -> Self {
        Self {
            names: Names::default(),
        }
    }
}

impl From<User> for NameHistoryResponse {
    fn from(user: User) -> Self {
        let names = user.extra.into();
        Self { names }
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
    let user = service::user::find_by_uid(uid.clone()).await?;
    if let Some(user) = user {
        Ok(Json(user.into()))
    } else {
        let user = collector::user::init_user_data(uid).await?;
        return Ok(Json(user.into()));
    }
}
