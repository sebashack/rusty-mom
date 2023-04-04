use sqlx::pool::PoolConnection;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::Postgres;
use sqlx::Pool;

pub type PoolConnectionPtr = PoolConnection<Postgres>;

pub struct DbPool {
    pool: Pool<Postgres>,
}

impl DbPool {
    pub async fn new(
        host: &str,
        port: u16,
        user: &str,
        db: &str,
        password: &str,
        min_conns: u32,
        max_conns: u32,
    ) -> Self {
        let conn_str = format!("postgresql://{user}:{password}@{host}:{port}/{db}");
        let pool = PgPoolOptions::new()
            .max_connections(max_conns)
            .min_connections(min_conns)
            .connect(conn_str.as_str())
            .await
            .unwrap();

        DbPool { pool }
    }

    pub async fn acquire(&self) -> PoolConnectionPtr {
        self.pool.acquire().await.unwrap()
    }
}
