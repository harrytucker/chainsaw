//! Provides a HTTP handler for exposing metrics to a Prometheus scraper.

use chainsaw::Result;

use axum::Extension;
use hyper::StatusCode;
use prometheus::{Counter, Opts, Registry, TextEncoder};

/// HTTP endpoint that exposes all registered metrics to a Prometheus scrape
/// run.
///
/// In order to gather metrics from the registry, you should combine
/// [`axum::Extension`] with a layer to expose your registry to this handler.
///
/// # Example
///
/// ```
/// let metrics_registry = prometheus::Registry::new();
/// // register any metrics
///
/// let http_router: axum::Router = axum::Router::new()
///     .layer(axum::Extension(metrics_registry));
/// ```
pub async fn prometheus_scrape_handler(
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
    use axum::{body::Body, http::Request, routing, Router};
    use tower::ServiceExt;

    use super::*;

    fn test_router(registry: Registry, metric: Counter) -> Router {
        Router::new()
            .route("/metrics", routing::get(prometheus_scrape_handler))
            .layer(Extension(registry))
            .layer(Extension(metric))
    }

    #[tokio::test]
    async fn metrics_reported() -> Result<()> {
        // Create a Prometheus registry and register an example metric. Sharing
        // across threads is fine as both Registry and Counter are `Send + Sync`.
        let registry = Registry::new();
        let counter = new_example_counter(
            "example_counter",
            "Reflects the number of times the greeter endpoint has been called.",
        )?;

        registry.register(Box::new(counter.clone()))?;
        let expected_metric_count = 5 as f64;
        counter.inc_by(expected_metric_count);

        let router = test_router(registry, counter);
        let response = router
            .oneshot(Request::builder().uri("/metrics").body(Body::empty())?)
            .await?;

        let body = hyper::body::to_bytes(response.into_body()).await?;
        assert_eq!(
            &body[..],
            b"# HELP example_counter Reflects the number of times the greeter endpoint has been called.\n\
              # TYPE example_counter counter\n\
              example_counter 5\n\
        ");

        Ok(())
    }
}
