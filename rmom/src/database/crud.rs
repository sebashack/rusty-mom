use sqlx::types::time::PrimitiveDateTime;
use sqlx::types::uuid::Uuid;
use sqlx::{self};
use time::Duration;

use super::connection::PoolConnectionPtr;

#[derive(Debug)]
pub struct MessageRecord {
    pub id: Uuid,
    pub queue_label: String,
    pub topic: String,
    pub content: Vec<u8>,
}

#[derive(Debug)]
pub struct QueueRecord {
    pub id: Uuid,
    pub label: String,
    pub mom_id: Option<Uuid>,
}

pub async fn insert_message(
    conn: &mut PoolConnectionPtr,
    id: &Uuid,
    queue_label: &str,
    topic: &str,
    ttl: i64,
    content: Vec<u8>,
) {
    let expires_at = sql_timestamp().saturating_add(Duration::seconds(ttl));
    sqlx::query!(
        "INSERT INTO message (id, queue_label, topic, expires_at, content) VALUES ($1, $2, $3, $4, $5)",
        id,
        queue_label,
        topic,
        expires_at,
        content
    )
    .execute(conn)
    .await
    .unwrap();
}

pub async fn select_queue_non_expired_messages(
    conn: &mut PoolConnectionPtr,
    queue_label: &str,
) -> Vec<MessageRecord> {
    sqlx::query_as!(
        MessageRecord,
        "SELECT id, queue_label, topic, content FROM message WHERE queue_label = $1 AND expires_at > NOW() ORDER BY created_at ASC",
        queue_label,
    )
    .fetch_all(conn)
    .await
    .unwrap()
}

pub async fn select_queues_by_mom(
    conn: &mut PoolConnectionPtr,
    host: &str,
    port: i32,
) -> Vec<QueueRecord> {
    sqlx::query_as!(
        QueueRecord,
        "SELECT queue.id, label, mom_id FROM queue INNER JOIN mom as m ON queue.mom_id = m.id WHERE m.host = $1 AND m.port = $2",
        host,
        port
    )
    .fetch_all(conn)
    .await
    .unwrap()
}

// Helpers
fn sql_timestamp() -> PrimitiveDateTime {
    let now_utc = sqlx::types::time::OffsetDateTime::now_utc();
    sqlx::types::time::PrimitiveDateTime::new(now_utc.date(), now_utc.time())
}