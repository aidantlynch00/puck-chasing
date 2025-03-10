mod args;
mod types;
mod db;
mod app;

use std::process::ExitCode;
use std::sync::Arc;
use clap::Parser;
use args::Args;
use dotenv::{from_path, var};
use tokio::net::TcpListener;
use reqwest::Client;
use db::pool::ConnectionPool;
use db::conn::DatabaseConnection;
use axum::serve;
use app::AppState;
use app::routes::router;

#[tokio::main]
async fn main() -> ExitCode {
    // parse the command line arguments
    let args = Args::parse();

    // load environment variables from the env file
    if let Err(env_err) = from_path(args.env) {
        eprintln!("Could not load environment variables: {env_err}");
        return ExitCode::FAILURE;
    }

    // grab the database path from the environment
    let db_path = match var("DATABASE_PATH") {
        Ok(path) => path,
        Err(env_err) => {
            eprintln!("Could not get the database path: {env_err}");
            return ExitCode::FAILURE;
        }
    };

    // open a database connection pool
    let pool = match ConnectionPool::open(db_path).await {
        Ok(pool) => pool,
        Err(db_err) => {
            eprintln!("Could not open the database: {db_err}");
            return ExitCode::FAILURE;
        }
    };

    // get connection from the pool
    let mut conn = match pool.conn().await {
        Ok(conn) => conn,
        Err(_timeout) => {
            eprintln!("Timeout while getting database connection");
            return ExitCode::FAILURE;
        }
    };

    // create the database tables if they do not exist
    if let Err(table_err) = conn.create_tables().await {
        eprintln!("Could not create tables: {table_err}");
        return ExitCode::FAILURE;
    }

    // close the initial connection
    drop(conn);

    let client = match Client::builder().build() {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Could not create HTTP client: {err}");
            return ExitCode::FAILURE;
        }
    };

    let app_state = AppState { client, pool };
    let app_state = Arc::new(app_state);

    let listener = match TcpListener::bind(format!("0.0.0.0:{}", args.port)).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Could not bind TCP listener: {err}");
            return ExitCode::FAILURE;
        }
    };

    match serve(listener, router(app_state)).await {
        Ok(()) => return ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error running app: {err}");
            return ExitCode::FAILURE;
        }
    }
}
