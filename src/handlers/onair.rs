use std::{collections::HashMap, sync::Arc};

use crate::error::AppError;
use crate::models::onair::BangumiItem;
use crate::states::AppState;
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

pub async fn handler(
    Query(pagination): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<OnAirResponse>, AppError> {
    let mut items = Vec::new();
    if let Some(subjects) = pagination.get("subjects") {
        let subjects = subjects.split(",").collect::<Vec<&str>>();
        if subjects.len() > 0 {
            let onair = state.onair.read().await;
            for id in subjects {
                if let Some(item) = onair.get(id) {
                    items.push(OnAirResultTuple(id.parse::<usize>().unwrap(), item.clone()))
                }
            }
        }
    }
    Ok(Json(OnAirResponse { data: items }))
}
