use std::sync::Arc;
use std::collections::HashMap;
use axum::Router;
use axum::routing::{get, post};
use axum::extract::{Path, State};
use axum::response::Json;
use axum::http::StatusCode;
use crate::types::string::{PlayerId, Username};
use crate::db::conn::DatabaseConnection;
use crate::app::AppState;


pub fn router() -> Router<Arc<AppState>> {
    let router = Router::new()
        .route("/", post(post_player))
        .route("/names", get(get_player_names))
        .route("/ranks", get(get_player_ranks))
        .route("/matches", get(get_player_matches));

    Router::new()
        .nest("/player/{player_id}", router)
        .route("/players", get(get_all_player_names))
}

async fn get_player_names(
    Path(player_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> String {
    format!("This is where player {} names will be sent", player_id)
}

async fn get_all_player_names(
    State(state): State<Arc<AppState>>,
) -> Result<Json<HashMap<PlayerId, Vec<Username>>>, StatusCode> {
    let mut conn = state.pool.conn()
        .await
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;

    let all_player_names = conn.get_all_player_names()
        .await
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|(player_row, name_rows)| {
            let names = name_rows.into_iter()
                .map(|name_row| name_row.name)
                .collect::<Vec<Username>>();

            (player_row.slap_id, names)
        })
        .collect::<HashMap<PlayerId, Vec<Username>>>();

    Ok(Json(all_player_names))
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
