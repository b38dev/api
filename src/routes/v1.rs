use std::sync::Arc;

use crate::handlers;
use crate::states::AppState;
use axum::{routing::get, Router};

pub fn create_router() -> Router<Arc<AppState>> {
    let router = Router::<Arc<AppState>>::new();
    #[cfg(feature = "onair")]
    let router = router.route("/onair", get(handlers::onair::handler));
    #[cfg(feature = "user")]
    let router = router.route("/user", get(handlers::user::handler));
    router
}
