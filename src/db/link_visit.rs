use anyhow::Context;
use chrono::{DateTime, Utc};
use sqlx::{query, FromRow, PgPool};
use tracing::instrument;

#[derive(Debug, Clone, FromRow)]
pub struct LinkVisit {
    pub link_id: String,
    pub ts: DateTime<Utc>,
}

impl LinkVisit {
    #[instrument(skip(pool))]
    pub async fn count_for_link_id(pool: &PgPool, link_id: i64) -> anyhow::Result<i64> {
        query!("Select count(*) as cnt From link_visits Where link_id = $1", link_id)
            .fetch_one(pool)
            .await
            .with_context(|| format!("Failed to count link visits for {link_id}"))
            .map(|row| row.cnt.unwrap_or(0))
    }
}
