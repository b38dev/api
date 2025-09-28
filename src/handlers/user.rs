use std::{collections::HashMap, sync::Arc};

use crate::error::AppError;
use crate::states::AppState;
use axum::{
    Json,
    extract::{Query, State},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {}

pub async fn handler(
    Query(pagination): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserResponse>, AppError> {
    Ok(Json(UserResponse {}))
}
