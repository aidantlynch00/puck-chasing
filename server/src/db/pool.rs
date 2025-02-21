use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::{Pool, RunError};
use crate::db::conn::{AsyncSqliteConnection, Connection};

static POOL_MAX_CONNS: usize = 10;


#[derive(Clone, Debug)]
pub struct ConnectionPool {
    pool: Pool<AsyncSqliteConnection>,
}

impl ConnectionPool {
    pub async fn open<S: Into<String>>(path: S) -> Result<Self, RunError> {
        let manager = AsyncDieselConnectionManager::<AsyncSqliteConnection>::new(path);
        let pool = Pool::builder()
            .max_size(POOL_MAX_CONNS as u32)
            .build(manager)
            .await?;

        Ok(Self { pool })
    }

    pub async fn conn(&self) -> Result<Connection, RunError> {
        let conn = self.pool.get().await?;
        Ok(Connection::from(conn))
    }
}

#[cfg(test)]
mod tests {
    use diesel_async::pooled_connection::bb8::RunError;
    use super::*;

    async fn open_in_memory() -> Result<ConnectionPool, RunError> {
        ConnectionPool::open(":memory:").await
    }

    async fn take_all_conns(pool: &ConnectionPool) -> Vec<Connection> {
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
        println!("{:?}", pool_res);
        assert!(pool_res.is_err());
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
        assert!(conn_res.is_err());
    }

    #[tokio::test]
    async fn create_tables() {
        todo!();
    }

    #[tokio::test]
    async fn tables_exist() {
        todo!();
    }

    #[tokio::test]
    async fn add_player() {
        todo!();
    }

    #[tokio::test]
    async fn player_exists() {
        todo!();
    }
}
