use crate::Result;

use axum::extract::Extension;
use hyper::StatusCode;
use prometheus::{Counter, Encoder, Opts, Registry, TextEncoder};

pub async fn report_metrics(
    Extension(metrics_registry): Extension<Registry>,
) -> Result<String, StatusCode> {
    // Gather the metrics.
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = metrics_registry.gather();
    if let Err(_) = encoder.encode(&metric_families, &mut buffer) {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    match String::from_utf8(buffer) {
        Ok(metrics) => Ok(metrics),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn new_example_counter<S: Into<String>>(name: S, help: S) -> Result<Counter> {
    let opts = Opts::new(name.into(), help.into());
    Ok(Counter::with_opts(opts)?)
}
