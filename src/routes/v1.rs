use std::sync::Arc;

use crate::handlers;
use crate::states::AppState;
use axum::{routing::get, Router};

pub fn create_router() -> Router<Arc<AppState>> {
    Router::<Arc<AppState>>::new().route("/onair", get(handlers::onair::handler))
}
