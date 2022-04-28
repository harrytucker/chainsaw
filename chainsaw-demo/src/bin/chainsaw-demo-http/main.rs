use crate::metrics::report_metrics;
use axum::{routing, AddExtensionLayer, Router};
use chainsaw::{config::get_configuration, Result};
use chainsaw_observe::logging;
use prometheus::{Counter, Registry};
use tokio::signal;

mod greeter;
mod metrics;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = get_configuration()?;

    let subscriber = logging::new_subscriber(configuration.log.level)?;
    logging::set_global_logger(subscriber)?;

    // Create a Prometheus registry and register an example metric. Sharing
    // across threads is fine as both Registry and Counter are `Send + Sync`.
    let metrics_registry = Registry::new();
    let example_counter = metrics::new_example_counter(
        "example_counter",
        "Reflects the number of times the greeter endpoint has been called.",
    )?;
    metrics_registry.register(Box::new(example_counter.clone()))?;

    // Create HTTP router with greeter and metric endpoints.
    let http_addr = configuration.http.unwrap().serve_addr();
    let http_router = app(metrics_registry, example_counter);
    let http = axum::Server::bind(&http_addr).serve(http_router.into_make_service());

    tracing::info!(?http, "Revving up HTTP Chainsaw!");
    tokio::spawn(http);

    // Exit if SIGINT received.
    signal::ctrl_c().await?;
    tracing::info!("Revving down Chainsaw...");
    Ok(())
}

pub fn app(registry: Registry, metric: Counter) -> Router {
    Router::new()
        .route("/:name/:surname", routing::get(greeter::greeter))
        .route("/metrics", routing::get(report_metrics))
        .layer(AddExtensionLayer::new(registry))
        .layer(AddExtensionLayer::new(metric))
        .layer(logging::http_trace_layer())
}
