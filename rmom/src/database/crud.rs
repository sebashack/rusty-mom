use sqlx::types::uuid::Uuid;
use sqlx::{self};

use super::connection::PoolConnectionPtr;

pub async fn insert_message(
    conn: &mut PoolConnectionPtr,
    id: &Uuid,
    queue_label: &str,
    topic: &str,
    ttl: i32,
    content: Vec<u8>,
) {
    sqlx::query!(
        "INSERT INTO message (id, queue_label, topic, ttl, content) VALUES ($1, $2, $3, $4, $5)",
        id,
        queue_label,
        topic,
        ttl,
        content
    )
    .execute(conn)
    .await
    .unwrap();
}
