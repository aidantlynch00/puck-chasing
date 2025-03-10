pub mod routes;
pub mod jobs;

use reqwest::Client;
use crate::db::pool::ConnectionPool;

pub struct AppState {
    pub client: Client,
    pub pool: ConnectionPool,
}
