mod errors;

use std::net::SocketAddr;

use anyhow::Context;
use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer};
use tracing::info;

use crate::context::AppState;

pub async fn serve(bind: &SocketAddr, ctx: AppState) -> anyhow::Result<()> {
    let app = http_router(ctx.clone()).layer(
        // layers are constructed from inner to outer (the last added one being the most outer one)
        ServiceBuilder::new()
            // some niceties
            .layer(NormalizePathLayer::trim_trailing_slash()) // make context available in handlers
            // enable logging of http requests
            .layer(TraceLayer::new_for_http()),
    );

    info!("Start http server on {}", bind);
    axum::Server::bind(bind)
        .serve(app.into_make_service())
        .await
        .context("Error starting HTTP server!")
}

fn http_router(ctx: AppState) -> Router {
    Router::new()
        // tell bots: this site is not for them
        .route("/robots.txt", get(|| async { "User-agent: *\nDisallow: /" }))
        // .route("/:link_id", get(links::show_link_route).post(links::redeem_link_route))
        // .nest_service("/-/assets", statics_dir)
        .fallback(errors::handle_404)
        // make context available in handlers
        .with_state(ctx)
}
