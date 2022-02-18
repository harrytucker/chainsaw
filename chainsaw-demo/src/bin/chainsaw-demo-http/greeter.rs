use axum::extract::{Extension, Path};
use prometheus::Counter;

#[tracing::instrument]
pub async fn greeter(
    Path((name, surname)): Path<(String, String)>,
    Extension(counter_metric): Extension<Counter>,
) -> String {
    counter_metric.inc();
    format!("Hello, {name} {surname}!\n")
}
