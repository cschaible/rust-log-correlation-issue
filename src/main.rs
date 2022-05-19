use std::{env, net::SocketAddr};

use api::greet;
use axum::{routing::post, Router};
use opentelemetry::global;

use tower_http::trace::TraceLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

pub mod api;

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug");
    }

    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("greet_service")
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Couldn't initialize tracer");

    // Using this tracer instead of the jaeger tracer above prints the trace_id and span_id
    // This confuses me a little bit.
    //let tracer = opentelemetry::sdk::export::trace::stdout::new_pipeline().install_simple();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").expect("RUST_LOG env variable not set"),
        ))
        .with(tracing_subscriber::fmt::layer().json())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()
        .expect("Initialization of tracing failed");

    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/greet", post(greet));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    global::shutdown_tracer_provider();
}
