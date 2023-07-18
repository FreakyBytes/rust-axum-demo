use std::borrow::Cow;

use opentelemetry::sdk::trace::RandomIdGenerator;
use opentelemetry::sdk::{propagation::TraceContextPropagator, Resource};
use opentelemetry::{sdk::trace::Sampler, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::LevelFilter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
    Registry,
};

use crate::cli::{Args, GIT_VERSION_TAG};

pub(crate) async fn setup_tracing(args: &Args) {
    // ----------------------------------------
    //     Open Telemetry

    // this allows tracking spans across services through known HTTP headers
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    let telemetry_layer = if let Some(otlp_endpoint) = &args.otlp_endpoint {
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_trace_config(
                opentelemetry::sdk::trace::config()
                    // all traces will have the same service name!
                    .with_resource(Resource::new(vec![
                        KeyValue::new(
                            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                            crate::cli::PKG_NAME,
                        ),
                        KeyValue::new(
                            opentelemetry_semantic_conventions::resource::SERVICE_VERSION,
                            crate::cli::PKG_VERSION,
                        ),
                    ]))
                    // send all traces!
                    .with_sampler(Sampler::AlwaysOn)
                    .with_id_generator(RandomIdGenerator::default()),
            )
            .with_exporter(
                // use GRPC to upstream our traces
                opentelemetry_otlp::new_exporter().tonic().with_endpoint(otlp_endpoint),
            )
            // send stuff async
            .install_batch(opentelemetry::runtime::Tokio)
            .expect("Failed to install OTLP tracer with tokio runtime!");

        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        let tracing_filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .parse_lossy(args.trace_level.clone().unwrap_or("".to_string()));
        let telemetry_layer = ErrorLayer::default().and_then(telemetry).with_filter(tracing_filter);

        // cf. https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/index.html#runtime-configuration-with-layers
        Some(telemetry_layer)
    } else {
        None
    };

    // ----------------------------------------
    //     Sentry

    let (sentry_guard, sentry_tracing_layer) = match args.sentry_dsn {
        Some(ref dsn) => {
            let guard = sentry::init((
                dsn.to_string(),
                sentry::ClientOptions {
                    release: Some(Cow::Owned(GIT_VERSION_TAG.into())),
                    environment: std::env::var("SENTRY_ENVIRONMENT").ok().map(Cow::Owned),
                    traces_sample_rate: std::env::var("SENTRY_TRACE_SAMPLE_RATE")
                        .ok()
                        .and_then(|timeout| timeout.parse::<f32>().ok())
                        .unwrap_or(0.1),

                    ..Default::default()
                },
            ));

            (Some(guard), Some(sentry_tracing::layer()))
        }
        None => (None, None),
    };
    // Intentionally leak the sentry guard, since it must be hold until application stop
    std::mem::forget(sentry_guard);

    // ----------------------------------------
    //     tracing / logging init

    // separate filter for logging to stdout and to send via OTLP
    let stdout_log_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .parse_lossy(&args.log_level);

    // finally register the layer stack
    Registry::default()
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(true)
                .with_timer(tracing_subscriber::fmt::time::time())
                .with_filter(stdout_log_filter),
        )
        .with(telemetry_layer)
        .with(sentry_tracing_layer)
        .init();
}
