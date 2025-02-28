mod args;
mod types;
mod db;

use std::process::ExitCode;
use clap::Parser;
use args::Args;
use dotenv::{from_path, var};
use db::pool::ConnectionPool;
use db::conn::DatabaseConnection;

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

    ExitCode::SUCCESS
}
