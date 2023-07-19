use clap::Parser;
use tracing::{debug, error, info};

use crate::{
    cli::{Args, CliCommand},
    context::AppState,
};

mod cli;
mod context;
mod db;
mod routes;
mod telemetry;

#[tokio::main]
async fn main() {
    // Load dotenv from here or parent directory - ignore any error
    dotenvy::dotenv().ok();

    let args = Args::parse();
    telemetry::setup_tracing(&args).await;
    debug!("CMD args: {:#?}", args);

    info!("Init app context");
    let ctx = match AppState::new(args.clone()).await {
        Ok(ctx) => ctx,
        Err(err) => {
            error!(err = ?err, "Failed to init app context");
            std::process::exit(3);
        }
    };

    match args.command {
        CliCommand::Migrate => {
            info!("Run database migrations");
            match sqlx::migrate!().run(&ctx.pool).await {
                Ok(_) => info!("Migrations successfully applied!"),
                Err(err) => {
                    error!(err = ?err, "Failed to apply migrations!");
                    std::process::exit(2);
                }
            }
        }
        CliCommand::Serve { ref http_bind } => {
            if let Err(err) = routes::serve(http_bind, ctx).await {
                error!(err = ?err, "Failed to start HTTP server!");
                std::process::exit(1);
            }
        }
    }
}
