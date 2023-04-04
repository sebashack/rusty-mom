use sqlx::types::uuid::Uuid;
use sqlx::{self};

use super::connection::PoolConnectionPtr;

pub async fn insert_message(
    conn: &mut PoolConnectionPtr,
    id: &Uuid,
    queue_id: &Uuid,
    topic: &str,
    ttl: i32,
) {
    sqlx::query!(
        "INSERT INTO message (id, queue_id, topic, ttl) VALUES ($1, $2, $3, $4)",
        id,
        queue_id,
        topic,
        ttl,
    )
    .execute(conn)
    .await
    .unwrap();
}
