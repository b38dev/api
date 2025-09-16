use crate::handlers;
use crate::state::AppState;
use axum::{routing::get, Router};

pub fn create_router() -> Router<AppState> {
    Router::<AppState>::new().route("/onair", get(handlers::onair_handler))
}
