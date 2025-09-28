use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::error::AppError;
use crate::models::onair::BangumiItem;
use crate::states::AppState;
use axum::{
    extract::{Query, State},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct OnAirResponse {
    pub data: Vec<(usize, BangumiItem)>,
}

pub async fn handler(
    Query(pagination): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<OnAirResponse>, AppError> {
    let mut items = Vec::new();
    if let Some(subjects) = pagination.get("subjects") {
        let subjects = subjects.split(",").collect::<HashSet<&str>>();
        if subjects.len() > 0 {
            for id in subjects {
                if let Some(item) = state.onair.get(id).await {
                    items.push((id.parse::<usize>().unwrap(), item))
                }
            }
        }
    }
    Ok(Json(OnAirResponse { data: items }))
}
