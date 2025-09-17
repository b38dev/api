pub mod onair;
use crate::states::AppState;
use std::sync::Arc;

pub fn start_tasks(app_state: Arc<AppState>) {
    onair::refresh(app_state);
}
