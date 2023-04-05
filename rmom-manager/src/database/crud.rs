use sqlx::types::uuid::Uuid;
use sqlx::{self};

use super::connection::PoolConnectionPtr;

use crate::utils::time::sql_timestamp;

pub struct TopicRecord {
    pub topic: String,
}
#[derive(Debug)]
pub struct ChannelRecord {
    pub id: Uuid,
    pub queue_id: Uuid,
    pub topic: String,
}

#[derive(Debug)]
pub struct QueueRecord {
    pub id: Uuid,
    pub label: String,
    pub mom_id: Option<Uuid>,
}

#[derive(Debug)]
pub struct MoMRecord {
    pub id: Uuid,
    pub host: String,
    pub port: i32,
    pub is_up: bool,
}

pub async fn select_all_moms(conn: &mut PoolConnectionPtr) -> Vec<MoMRecord> {
    sqlx::query_as!(MoMRecord, "SELECT id, host, port, is_up FROM mom")
        .fetch_all(conn)
        .await
        .unwrap()
}

pub async fn select_all_queues(conn: &mut PoolConnectionPtr) -> Vec<QueueRecord> {
    sqlx::query_as!(QueueRecord, "SELECT id, label, mom_id FROM queue")
        .fetch_all(conn)
        .await
        .unwrap()
}

pub async fn select_all_channels(conn: &mut PoolConnectionPtr) -> Vec<ChannelRecord> {
    sqlx::query_as!(ChannelRecord, "SELECT id, queue_id, topic FROM channel")
        .fetch_all(conn)
        .await
        .unwrap()
}

pub async fn select_channel(
    conn: &mut PoolConnectionPtr,
    channel_id: &Uuid,
) -> Option<ChannelRecord> {
    sqlx::query_as!(
        ChannelRecord,
        "SELECT id, queue_id, topic FROM channel WHERE id = $1",
        channel_id,
    )
    .fetch_optional(conn)
    .await
    .unwrap()
}

pub async fn select_all_topics_by_queue_label(
    conn: &mut PoolConnectionPtr,
    queue_label: &str,
) -> Vec<TopicRecord> {
    sqlx::query_as!(
        TopicRecord,
        "SELECT channel.topic FROM channel INNER JOIN queue ON channel.queue_id = queue.id WHERE queue.label = $1",
        queue_label
    )
    .fetch_all(conn)
    .await
    .unwrap()
}

pub async fn select_queue(conn: &mut PoolConnectionPtr, queue_label: &str) -> Option<QueueRecord> {
    sqlx::query_as!(
        QueueRecord,
        "SELECT id, label, mom_id FROM queue WHERE label = $1",
        queue_label
    )
    .fetch_optional(conn)
    .await
    .unwrap()
}

pub async fn select_queue_by_id(
    conn: &mut PoolConnectionPtr,
    queue_id: &Uuid,
) -> Option<QueueRecord> {
    sqlx::query_as!(
        QueueRecord,
        "SELECT id, label, mom_id FROM queue WHERE id = $1",
        queue_id
    )
    .fetch_optional(conn)
    .await
    .unwrap()
}

pub async fn select_if_queue_exists(conn: &mut PoolConnectionPtr, label: &str) -> bool {
    sqlx::query!("SELECT id FROM queue WHERE label = $1", label)
        .fetch_optional(conn)
        .await
        .unwrap()
        .is_some()
}

pub async fn select_mom(conn: &mut PoolConnectionPtr, mom_id: &Uuid) -> Option<MoMRecord> {
    sqlx::query_as!(
        MoMRecord,
        "SELECT id, host, port, is_up FROM mom WHERE id = $1",
        mom_id
    )
    .fetch_optional(conn)
    .await
    .unwrap()
}

pub async fn insert_mom(
    conn: &mut PoolConnectionPtr,
    id: &Uuid,
    host: &str,
    port: i32,
    is_up: bool,
) {
    sqlx::query!(
        "INSERT INTO mom (id, host, port, is_up) VALUES ($1, $2, $3, $4) ON CONFLICT (host, port) DO UPDATE SET is_up = $4, updated_at = $5",
        id,
        host,
        port,
        is_up,
        sql_timestamp(),
    )
    .execute(conn)
    .await
    .unwrap();
}

pub async fn update_mom_is_up(
    conn: &mut PoolConnectionPtr,
    mom_id: &sqlx::types::uuid::Uuid,
    is_up: bool,
) {
    sqlx::query!(
        "UPDATE mom SET is_up=$2, updated_at=$3 WHERE id = $1",
        mom_id,
        is_up,
        sql_timestamp(),
    )
    .execute(conn)
    .await
    .unwrap();
}

pub async fn insert_queue(
    conn: &mut PoolConnectionPtr,
    queue_id: &Uuid,
    label: &str,
    mom_id: &Uuid,
) {
    sqlx::query!(
        "INSERT INTO queue (id, label, mom_id) VALUES ($1, $2, $3)",
        queue_id,
        label,
        mom_id,
    )
    .execute(conn)
    .await
    .unwrap();
}

pub async fn insert_channel(conn: &mut PoolConnectionPtr, id: &Uuid, queue_id: &Uuid, topic: &str) {
    sqlx::query!(
        "INSERT INTO channel (id, queue_id, topic) VALUES ($1, $2, $3)",
        id,
        queue_id,
        topic,
    )
    .execute(conn)
    .await
    .unwrap();
}

pub async fn update_queue_mom(
    conn: &mut PoolConnectionPtr,
    queue_id: &sqlx::types::uuid::Uuid,
    mom_id: &sqlx::types::uuid::Uuid,
) {
    sqlx::query!(
        "UPDATE queue SET mom_id=$2, updated_at=$3 WHERE id = $1",
        queue_id,
        mom_id,
        sql_timestamp(),
    )
    .execute(conn)
    .await
    .unwrap();
}

pub async fn delete_queue(conn: &mut PoolConnectionPtr, id: &Uuid) {
    sqlx::query!("DELETE FROM queue WHERE id = $1", id)
        .execute(conn)
        .await
        .unwrap();
}

pub async fn delete_channel(conn: &mut PoolConnectionPtr, id: &Uuid) {
    sqlx::query!("DELETE FROM channel WHERE id = $1", id)
        .execute(conn)
        .await
        .unwrap();
}
