use std::sync::Arc;
use std::collections::HashMap;
use axum::Router;
use axum::routing::{get, post};
use axum::extract::{Path, Query, State};
use axum::response::Json;
use axum::http::StatusCode;
use serde_json::{json, Value};
use diesel::result::Error;
use crate::types::string::{PlayerId, Username};
use crate::db::conn::DatabaseConnection;
use crate::app::AppState;
use crate::app::routes::Pagination;
use crate::app::jobs::get_recent_history;

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
    Path(player_id): Path<&str>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    let mut conn = state.pool.conn()
        .await
        .map_err(|_pool_timeout| {
            eprintln!("Could not get database connection from pool");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let player_id = PlayerId::from(player_id);
    match conn.get_player_id(&player_id).await {
        Ok(Some(player_row)) => match conn.get_player_names(&player_row).await {
            Ok(name_rows) => {
                let names = name_rows.into_iter()
                    .map(|name_row| name_row.name)
                    .collect::<Vec<Username>>();

                return Ok(Json(json!(names)));
            },
            Err(db_err) => {
                eprintln!(
                    "Database error when querying names for player with ID '{}': {}",
                    player_id,
                    db_err,
                );

                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        },
        Ok(None) => {
            eprintln!("Player with ID '{}' could not be found", player_id);
            return Err(StatusCode::NOT_FOUND);
        },
        Err(db_err) => {
            eprintln!(
                "Database error when querying for player with ID '{}': {}",
                player_id,
                db_err,
            );

            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

async fn get_all_player_names(
    State(state): State<Arc<AppState>>,
) -> Result<Json<HashMap<PlayerId, Vec<Username>>>, StatusCode> {
    let mut conn = state.pool.conn()
        .await
        .map_err(|_pool_timeout| {
            eprintln!("Could not get database connection from pool");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

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
    Path(player_id): Path<&str>,
    State(state): State<Arc<AppState>>,
) -> String {
    format!("This is where player {} ranks will be sent", player_id)
}

async fn get_player_matches(
    Path(player_id): Path<&str>,
    Query(pagination): Query<Pagination>,
    State(state): State<Arc<AppState>>,
) -> Result<(), StatusCode> {
    let mut conn = state.pool.conn()
        .await
        .map_err(|_pool_timeout| {
            eprintln!("Could not get database connection from pool");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let player_id = PlayerId::from(player_id);
    match conn.get_player_id(&player_id).await {
        Ok(Some(player_row)) => match conn.get_player_matches(&player_row).await {
            Ok(match_rows) => {
                // TODO: need structure for match info + players
                let matches = match_rows.into_iter()
                    .map()
                    .collect::<>();
            },
            Err(db_err) => {
                todo!();
            }
        },
        Ok(None) => {
            todo!();
        },
        Err(db_err) => {
            todo!();
        }
    };
}

async fn post_player(
    Path(player_id): Path<&str>,
    State(state): State<Arc<AppState>>,
) -> StatusCode {
    let player_id = PlayerId::from(player_id);

    // attempt to request this player's recent history
    match get_recent_history(&state.client, &player_id).await {
        Ok(history) => {
            // use recent history to populate the database
            let mut conn = match state.pool.conn().await {
                Ok(conn) => conn,
                Err(_pool_timeout) => {
                    eprintln!("Could not get database connection from pool");
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }
            };

            // check if this player already exists
            match conn.get_player_id(&history.player.game_user_id).await {
                Ok(Some(player_row)) => {
                    todo!();
                },
                Ok(None) => {
                    todo!();
                },
                Err(db_err) => {
                    eprintln!("Could not query player by ID: {}", db_err);
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }
            };

            return StatusCode::OK;
        },
        Err(req_err) => {
            eprintln!("Could not get player's recent history: {}", req_err);
            let status = match req_err.status() {
                Some(status) => status,
                None => StatusCode::NOT_FOUND
            };

            return status;
        },
    }
}
