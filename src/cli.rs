use std::net::SocketAddr;

use clap::{Parser, Subcommand};

pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const GIT_VERSION_TAG: &str = env!("GIT_VERSION_TAG");

#[derive(Debug, Clone, Parser)]
#[clap(name = PKG_NAME, author = AUTHORS, version = GIT_VERSION_TAG, about)]
pub struct Args {
    #[clap(short = 'l', long, env = "LOG_LEVEL", default_value = "info")]
    pub log_level: String,
    #[clap(long, env = "TRACE_LEVEL")]
    pub trace_level: Option<String>,
    #[clap(long, env = "OTLP_ENDPOINT")]
    pub otlp_endpoint: Option<String>,
    #[clap(long, env = "SENTRY_DSN")]
    pub sentry_dsn: Option<String>,

    /// URL to the PostgreSQL database
    #[clap(short = 'd', long, env = "DATABASE_URL")]
    pub database_url: String,
    #[clap(long, env = "MAX_POOL_SIZE", default_value_t = 10)]
    pub max_db_conn_pool_size: u32,
    #[clap(long, env = "CONN_IDLE_TIMEOUT", default_value = "15min")]
    pub conn_idle_timeout: Option<humantime::Duration>,
    #[clap(long, env = "CONN_LIFETIME")]
    pub conn_lifetime: Option<humantime::Duration>,

    #[clap(subcommand)]
    pub command: CliCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum CliCommand {
    /// Start the http server
    Serve {
        #[clap(long, env = "HTTP_BIND", default_value = "127.0.0.1:42069")]
        http_bind: SocketAddr,
    },
    /// Run the sql migrations
    Migrate,
}
