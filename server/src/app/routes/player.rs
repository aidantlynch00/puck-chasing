use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use crate::app::AppState;

pub fn router() -> Router<Arc<AppState>> {
    let router = Router::new()
        .route("/", post(post_player))
        .route("/names", get(get_player_names))
        .route("/ranks", get(get_player_ranks))
        .route("/matches", get(get_player_matches));

    Router::new().nest("/player/{player_id}", router)
}

async fn get_player_names(
    Path(player_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> String {
    format!("This is where player {} names will be sent", player_id)
}

async fn get_player_ranks(
    Path(player_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> String {
    format!("This is where player {} ranks will be sent", player_id)
}

async fn get_player_matches(
    Path(player_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> String {
    format!("This is where player {} matches will be sent", player_id)
}

async fn post_player(
    Path(player_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> StatusCode {
    StatusCode::OK
}
