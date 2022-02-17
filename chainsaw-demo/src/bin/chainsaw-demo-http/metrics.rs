//! Provides a HTTP handler for exposing metrics to a Prometheus scraper.

use crate::Result;

use axum::extract::Extension;
use hyper::StatusCode;
use prometheus::{Counter, Encoder, Opts, Registry, TextEncoder};

/// HTTP endpoint that exposes all registered metrics to a Prometheus scrape
/// run.
///
/// In order to gather metrics from the registry, you should use
/// [`axum::AddExtensionLayer`] to expose your registry to this handler.
///
/// # Example
///
/// ```
/// let metrics_registry = Registry::new();
/// // register any metrics
///
/// let http_router = Router::new()
///     // add any routes or other layers
///     .layer(AddExtensionLayer::new(metrics_registry));
/// ```
pub async fn report_metrics(
    Extension(metrics_registry): Extension<Registry>,
) -> Result<String, StatusCode> {
    // Gather the metrics.
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = metrics_registry.gather();
    if encoder.encode(&metric_families, &mut buffer).is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    match String::from_utf8(buffer) {
        Ok(metrics) => Ok(metrics),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Returns a new Prometheus counter with a given name and help string.
pub fn new_example_counter<S: Into<String>>(name: S, help: S) -> Result<Counter> {
    let opts = Opts::new(name.into(), help.into());
    Ok(Counter::with_opts(opts)?)
}
