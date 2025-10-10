use crate::AppState;
use axum::Router;

pub mod onair;
pub mod user;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/onair", onair::routes())
        .nest("/user", user::routes())
}
