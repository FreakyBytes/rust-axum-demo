use std::{ops::Deref, str::FromStr, sync::Arc, time::Duration};

use anyhow::Context;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};
use tracing::instrument;

use crate::cli::{Args, PKG_NAME};

///
/// Main application context container.
/// All data resides in an Arc, so this struct is easily cloneable
///
#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

impl AppState {
    #[instrument(skip_all, name = "create_app_context")]
    pub async fn new(args: Args) -> anyhow::Result<Self> {
        Ok(Self {
            inner: Arc::new(AppStateInner::new(args).await?),
        })
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug)]
pub struct AppStateInner {
    pub args: Args,
    pub pool: Pool<Postgres>,
}

impl AppStateInner {
    async fn new(args: Args) -> anyhow::Result<Self> {
        let mut db_options = PgConnectOptions::from_str(&args.database_url)?.application_name(PKG_NAME);
        db_options.log_statements(tracing::log::LevelFilter::Debug);
        db_options.log_slow_statements(tracing::log::LevelFilter::Warn, Duration::from_millis(250));

        let db_pool = PgPoolOptions::new()
            .max_connections(args.max_db_conn_pool_size)
            .idle_timeout(args.conn_idle_timeout.map(|t| t.into()))
            .max_lifetime(args.conn_lifetime.map(|t| t.into()))
            .connect_with(db_options)
            .await
            .context("Failed to create DB pool")?;

        Ok(Self { args, pool: db_pool })
    }
}
