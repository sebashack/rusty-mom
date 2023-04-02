pub fn sql_timestamp() -> sqlx::types::time::PrimitiveDateTime {
    let now_utc = sqlx::types::time::OffsetDateTime::now_utc();
    sqlx::types::time::PrimitiveDateTime::new(now_utc.date(), now_utc.time())
}
