use anyhow::Context;
use chrono::{DateTime, Utc};
use sqlx::{query, query_as, FromRow, PgPool};
use tracing::instrument;

#[derive(Debug, Clone, FromRow)]
pub struct LinkVisit {
    pub link_id: i32,
    pub ts: DateTime<Utc>,
}

impl LinkVisit {
    #[instrument(skip(pool))]
    pub async fn count_for_link_id(pool: &PgPool, link_id: i32) -> anyhow::Result<i64> {
        query!("Select count(*) as cnt From link_visits Where link_id = $1", link_id)
            .fetch_one(pool)
            .await
            .with_context(|| format!("Failed to count link visits for {link_id}"))
            .map(|row| row.cnt.unwrap_or(0))
    }

    #[instrument(skip(pool))]
    pub async fn mark_visit(pool: &PgPool, link_id: i32) -> anyhow::Result<LinkVisit> {
        query_as!(
            LinkVisit,
            "Insert Into link_visits (link_id) Values ($1) Returning *",
            link_id,
        )
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to mark visit for {link_id}"))
    }
}
