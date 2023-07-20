use anyhow::Context;
use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, PgPool};
use tracing::instrument;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Link {
    pub link_id: i32,
    pub code: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
}

impl Link {
    #[instrument(skip(pool))]
    pub async fn find_by_id(pool: &PgPool, link_id: i32) -> anyhow::Result<Option<Self>> {
        query_as!(Self, "Select * From links Where link_id = $1 Limit 1", link_id)
            .fetch_optional(pool)
            .await
            .context("Error fetching link by id")
    }

    #[instrument(skip(pool))]
    pub async fn find_by_code(pool: &PgPool, code: &str) -> anyhow::Result<Option<Self>> {
        query_as!(Self, "Select * From links Where code = $1 Limit 1", code)
            .fetch_optional(pool)
            .await
            .context("Error fetching link by id")
    }

    #[instrument(skip_all)]
    pub async fn create<S: ToString>(pool: &PgPool, url: &str, code: Option<S>) -> anyhow::Result<Self> {
        let code = code.map(|s| s.to_string()).unwrap_or_else(|| nanoid!());

        query_as!(
            Self,
            r#"Insert Into links (code, url) Values ($1, $2) Returning *"#,
            code,
            url
        )
        .fetch_one(pool)
        .await
        .context("Failed to create new link")
    }
}
