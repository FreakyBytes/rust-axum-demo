mod errors;
mod links;

use std::{net::SocketAddr, time::Instant};

use anyhow::Context;
use axum::{
    extract::{MatchedPath, State},
    http::Request,
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_tracing_opentelemetry::opentelemetry_tracing_layer;
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
            .layer(TraceLayer::new_for_http())
            .layer(opentelemetry_tracing_layer()),
    );

    info!("Starting http server on http://{}/", bind);
    axum::Server::bind(bind)
        .serve(app.into_make_service())
        .await
        .context("Error starting HTTP server!")
}

fn http_router(ctx: AppState) -> Router {
    Router::new()
        // tell bots: this site is not for them
        .route("/robots.txt", get(|| async { "User-agent: *\nDisallow: /" }))
        .route(
            "/metrics",
            get(move |State(ctx): State<AppState>| {
                // `std::future::ready` returns a Future that does not need to be awaited
                // it's a small trick to circumvent the overhead otherwise introduced by tokio polling the Future
                std::future::ready(ctx.prom_handle.render())
            }),
        )
        .nest("/api/links", links::router())
        .fallback(errors::handle_404)
        // inject axum middleware to track request duration
        .route_layer(middleware::from_fn(metric_middleware))
        // make context available in handlers
        .with_state(ctx)
}

async fn metric_middleware<Body>(req: Request<Body>, next: Next<Body>) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_string()
    } else {
        req.uri().path().to_string()
    };
    let method = req.method().clone().to_string();

    let resp = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = resp.status().as_u16().to_string();

    let labels = [("method", method), ("path", path), ("status", status)];

    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_requests_duration_seconds", latency, &labels);

    resp
}
