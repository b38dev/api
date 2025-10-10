use crate::AppState;

use axum::{Json, Router, extract::Query, routing::get};
use model::common::onair::{BangumiItem, BangumiItemMap, SubjectId, SubjectIds};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(query_by_subjects))
}

#[derive(serde::Serialize)]
pub struct Item((SubjectId, BangumiItem));

#[derive(serde::Serialize)]
pub struct OnAirResponse {
    pub data: Vec<Item>,
}

impl OnAirResponse {
    pub fn empty() -> Self {
        Self { data: Vec::new() }
    }
}

impl From<BangumiItemMap> for OnAirResponse {
    fn from(map: BangumiItemMap) -> Self {
        let data = map.into_iter().map(|(id, item)| Item((id, item))).collect();
        Self { data }
    }
}

#[derive(serde::Deserialize)]
pub struct OnAirQuery {
    #[serde(deserialize_with = "OnAirQuery::deserialize_subjects")]
    subjects: SubjectIds,
}

impl OnAirQuery {
    fn deserialize_subjects<'de, D>(deserializer: D) -> Result<SubjectIds, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        let subjects = s.split(',').filter_map(|v| v.parse().ok()).collect();
        Ok(subjects)
    }
}

#[axum::debug_handler]
pub async fn query_by_subjects(
    Query(OnAirQuery { subjects }): Query<OnAirQuery>,
) -> crate::Result<Json<OnAirResponse>> {
    if subjects.is_empty() {
        return Ok(Json(OnAirResponse::empty()));
    }
    let data = service::onair::find_by_subject_ids(&subjects).await?;
    Ok(Json(data.into()))
}
