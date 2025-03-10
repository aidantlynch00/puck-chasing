mod player;

use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::app::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    let api_router = Router::new()
        .merge(player::router())
        .with_state(state);

    Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .nest("/api", api_router)
}
