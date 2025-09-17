mod v1;
use std::sync::Arc;

use crate::states::AppState;
use axum::Router;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::<Arc<AppState>>::new().nest("/v1", v1::create_router())
}
