use std::collections::HashMap;

use crate::error::AppError;
use crate::models::BangumiItem;
use crate::state::AppState;
use axum::{
    extract::{Query, State},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct OnAirResultTuple(usize, BangumiItem);

#[derive(Serialize)]
pub struct OnAirResponse {
    pub data: Vec<OnAirResultTuple>,
}

pub async fn onair_handler(
    Query(pagination): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Json<OnAirResponse>, AppError> {
    let mut items = Vec::new();
    if let Some(q) = pagination.get("q") {
        let subjects = q.split(",").collect::<Vec<&str>>();
        if subjects.len() > 0 {
            let cache = state.subject_map.read().await;
            for id in subjects {
                if let Some(item) = cache.data.get(id) {
                    items.push(OnAirResultTuple(id.parse::<usize>().unwrap(), item.clone()))
                }
            }
        }
    }
    Ok(Json(OnAirResponse { data: items }))
}
