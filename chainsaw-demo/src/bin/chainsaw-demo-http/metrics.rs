//! Provides a HTTP handler for exposing metrics to a Prometheus scraper.

use crate::Result;

use axum::extract::Extension;
use hyper::StatusCode;
use prometheus::{Counter, Opts, Registry, TextEncoder};

/// HTTP endpoint that exposes all registered metrics to a Prometheus scrape
/// run.
///
/// In order to gather metrics from the registry, you should use
/// [`axum::AddExtensionLayer`] to expose your registry to this handler.
///
/// # Example
///
/// ```compile_fail
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
    // Create a new Prometheus text encoder, and gather all our metrics.
    let encoder = TextEncoder::new();
    let metric_families = metrics_registry.gather();

    match encoder.encode_to_string(&metric_families) {
        Ok(metrics) => Ok(metrics),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Returns a new Prometheus counter with a given name and help string.
pub fn new_example_counter<S: Into<String>>(name: S, help: S) -> Result<Counter> {
    let opts = Opts::new(name.into(), help.into());
    Ok(Counter::with_opts(opts)?)
}

#[cfg(test)]
mod test {
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    use super::*;
    use crate::app;

    #[tokio::test]
    async fn metrics_reported() {
        // Create a Prometheus registry and register an example metric. Sharing
        // across threads is fine as both Registry and Counter are `Send + Sync`.
        let registry = Registry::new();
        let counter = new_example_counter(
            "example_counter",
            "Reflects the number of times the greeter endpoint has been called.",
        )
        .unwrap();

        registry.register(Box::new(counter.clone())).unwrap();
        let expected_metric_count = 5 as f64;
        counter.inc_by(expected_metric_count);

        let app = app(registry, counter);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/metrics")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(
            &body[..],
            b"# HELP example_counter Reflects the number of times the greeter endpoint has been called.\n\
              # TYPE example_counter counter\n\
              example_counter 5\n\
        ")
    }
}
