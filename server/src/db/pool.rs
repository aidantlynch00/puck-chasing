use std::fmt::Display;
use tokio::fs::try_exists;
use diesel::result::ConnectionError;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, PoolError};
use diesel_async::pooled_connection::bb8::{Pool, RunError};
use crate::db::conn::{AsyncSqliteConnection, PooledSqliteConnection, ConnectionWrapper};

static POOL_MAX_CONNS: usize = 10;

#[derive(Debug)]
pub enum PoolCreationError {
    NotFound(String),
    Connection(ConnectionError),
}

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PoolCreationError::NotFound(path) =>
                f.write_fmt(format_args!("database \"{}\" not found", path)),
            PoolCreationError::Connection(conn_err) =>
                f.write_fmt(format_args!("database connection error ({})", conn_err)),
        }
    }
}

#[derive(Debug)]
pub struct PoolTimeout;

#[derive(Clone, Debug)]
pub struct ConnectionPool {
    pool: Pool<AsyncSqliteConnection>,
}

impl ConnectionPool {
    pub async fn open<S: Into<String>>(path: S) -> Result<Self, PoolCreationError> {
        // check that the path is an in-memory database or exists on the filesystem
        let path_str: String = path.into();
        match path_str.as_str() {
            ":memory:" => {},
            path_slice => match try_exists(path_slice).await {
                Ok(valid) if valid => {},
                _ => return Err(PoolCreationError::NotFound(path_str))
            }
        }

        let manager = AsyncDieselConnectionManager::<AsyncSqliteConnection>::new(path_str);
        let pool_res = Pool::builder()
            .max_size(POOL_MAX_CONNS as u32)
            .build(manager)
            .await;

        // assert that any failures are connection issues
        let pool = match pool_res {
            Ok(pool) => pool,
            Err(err) => match err {
                PoolError::ConnectionError(conn_err) =>
                    return Err(PoolCreationError::Connection(conn_err)),
                PoolError::QueryError(query_err) =>
                    panic!("should never be a query error when connecting! ({query_err})")
            }
        };

        Ok(Self { pool })
    }

    pub async fn conn(
        &self,
    ) -> Result<ConnectionWrapper<PooledSqliteConnection<'_>>, PoolTimeout> {
        // assert that any failures are a timeout
        let conn = match self.pool.get().await {
            Ok(conn) => conn,
            Err(run_err) => match run_err {
                RunError::TimedOut =>
                    return Err(PoolTimeout),
                RunError::User(pool_err) =>
                    panic!("should never be a connection or query error! ({pool_err})")
            }
        };

        Ok(ConnectionWrapper::from(conn))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn open_in_memory() -> Result<ConnectionPool, PoolCreationError> {
        ConnectionPool::open(":memory:").await
    }

    async fn take_all_conns(
        pool: &ConnectionPool,
    ) -> Vec<ConnectionWrapper<PooledSqliteConnection<'_>>> {
        let mut conns = Vec::new();
        for _ in 0..POOL_MAX_CONNS {
            let conn = pool.conn()
                .await
                .unwrap();

            conns.push(conn);
        }

        conns
    }

    #[tokio::test]
    async fn open_pool() {
        let pool_res = open_in_memory().await;
        assert!(pool_res.is_ok());
    }

    #[tokio::test]
    async fn database_dne() {
        let pool_res = ConnectionPool::open("/does/not/exist.db").await;
        assert!(matches!(pool_res, Err(PoolCreationError::NotFound(_))));
    }

    #[tokio::test]
    async fn conn_available() {
        let pool = open_in_memory()
            .await
            .unwrap();

        let conn_res = pool.conn().await;
        assert!(conn_res.is_ok());
    }

    #[tokio::test]
    async fn no_conn_available() {
        let pool = open_in_memory()
            .await
            .unwrap();

        let conns = take_all_conns(&pool).await;
        let conn_res = pool.conn().await;

        // explicitly hold connections until last connection future returns
        drop(conns);

        // test last connection result after dropping other connections to
        // avoid any leaks during unwind
        assert!(matches!(conn_res, Err(PoolTimeout)));
    }

}
