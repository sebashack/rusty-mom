use rocket_db_pools::{self, Connection, Database};
use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;

#[derive(Database)]
#[database("rmom_manager")]
pub struct Db(pub rocket_db_pools::sqlx::PgPool);

pub type DbConnection = Connection<Db>;
pub type PoolConnectionPtr = PoolConnection<Postgres>;
