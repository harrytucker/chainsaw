use crate::{config::get_configuration, metrics::report_metrics};
use axum::{routing, AddExtensionLayer, Router};
use chainsaw_demo::{logging, Result};
use prometheus::Registry;
use tokio::signal;

mod config;
mod greeter;
mod metrics;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = get_configuration()?;

    let subscriber = logging::new_subscriber(configuration.log.level)?;
    logging::set_global_logger(subscriber)?;

    let metrics_registry = Registry::new();
    let example_counter = metrics::new_example_counter(
        "example_counter",
        "Reflects the number of times the greeter endpoint has been called.",
    )?;
    metrics_registry.register(Box::new(example_counter.clone()))?;

    let http_addr = configuration.http.serve_addr();
    let http_router = Router::new()
        .route("/:name/:surname", routing::get(greeter::greeter))
        .route("/metrics", routing::get(report_metrics))
        .layer(AddExtensionLayer::new(metrics_registry))
        .layer(AddExtensionLayer::new(example_counter))
        .layer(logging::http_trace_layer());
    let http = axum::Server::bind(&http_addr).serve(http_router.into_make_service());

    tracing::info!(?http, "Revving up HTTP Chainsaw!");
    tokio::spawn(http);

    // Exit if SIGINT received.
    signal::ctrl_c().await?;
    tracing::info!("Revving down Chainsaw...");
    Ok(())
}
