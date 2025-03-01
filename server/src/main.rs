mod args;
mod api;
mod db;

use clap::Parser;
use args::Args;
use db::ConnectionPool;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let pool = ConnectionPool::open(args.database.to_string_lossy())
        .await
        .expect("could not create connection pool");

    if let Err(table_err) = pool.create_tables().await {
        println!("could not create tables: {table_err}");
    }
}
