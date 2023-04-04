use sqlx::types::time::PrimitiveDateTime;
use sqlx::types::uuid::Uuid;
use sqlx::{self};
use time::Duration;

use super::connection::PoolConnectionPtr;

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

// Helpers
fn sql_timestamp() -> PrimitiveDateTime {
    let now_utc = sqlx::types::time::OffsetDateTime::now_utc();
    sqlx::types::time::PrimitiveDateTime::new(now_utc.date(), now_utc.time())
}
